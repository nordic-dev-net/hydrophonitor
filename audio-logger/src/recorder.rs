use anyhow::anyhow;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use chrono::prelude::*;
use super::*;

type WriteHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

pub struct Recorder {
	writer: WriteHandle,
	interrupt_handles: InterruptHandles,
	default_config: SupportedStreamConfig,
	user_config: StreamConfig,
	device: Device,
	spec: hound::WavSpec,
	name: String,
	path: PathBuf,
	current_file: String,
}

fn get_host(host: HostId) -> Result<Host, anyhow::Error> {
	Ok(cpal::host_from_id(cpal::available_hosts()
		.into_iter()
		.find(|id| *id == host)
		.ok_or(anyhow!("Requested host device not found"))?
	)?)
}

fn get_device(host: Host) -> Result<Device, anyhow::Error> {
	Ok(host.default_input_device()
		.ok_or(anyhow!("No input device available. Try running `jackd -R -d alsa -d hw:0`",
	))?)
}

fn get_default_config(device: &Device) -> Result<SupportedStreamConfig, anyhow::Error> {
	Ok(device.default_input_config()?)
}

/// # Get User Config
///
/// Overrides certain fields of the default stream config with the user's config.
///
/// sample_rate: The user's sample rate if it is supported by the device, otherwise the default sample rate.
/// channels: The user's number of channels if it is supported by the device, otherwise the default number of channels.
/// buffer_size: The user's buffer size if it is supported by the device, otherwise the default buffer size.
fn get_user_config(sample_rate: u32, channels: u16, buffer_size: u32) -> Result<StreamConfig, anyhow::Error> {
	if !ALLOWED_SAMPLE_RATES.contains(&sample_rate) {
		return Err(anyhow!(
			"Sample rate {} is not supported. Allowed sample rates: {:?}",
			sample_rate,
			ALLOWED_SAMPLE_RATES
		));
	}
	if !(channels >= 1 && channels <= MAX_CHANNEL_COUNT) {
		return Err(anyhow!(
			"Channel count {} is not supported. Allowed channel counts: 1-{}",
			channels,
			MAX_CHANNEL_COUNT
		));
	}
	if !(buffer_size >= MIN_BUFFER_SIZE as u32 && buffer_size <= MAX_BUFFER_SIZE as u32) {
		return Err(anyhow!(
			"Buffer size {} is not supported. Allowed buffer sizes: {}-{}",
			buffer_size,
			MIN_BUFFER_SIZE,
			MAX_BUFFER_SIZE
		));
	}
	Ok(StreamConfig {
		channels,
		sample_rate: SampleRate(sample_rate),
		buffer_size: BufferSize::Fixed(buffer_size),
	})
}

fn get_spec(default_config: &SupportedStreamConfig, user_config: &StreamConfig) -> Result<hound::WavSpec, anyhow::Error> {
	Ok(hound::WavSpec {
		channels: user_config.channels,
		sample_rate: user_config.sample_rate.0,
		bits_per_sample: (default_config.sample_format().sample_size() * 8) as u16,
		sample_format: match default_config.sample_format() {
			cpal::SampleFormat::U16 => hound::SampleFormat::Int,
			cpal::SampleFormat::I16 => hound::SampleFormat::Int,
			cpal::SampleFormat::F32 => hound::SampleFormat::Float,
		},
	})
}

fn get_filename(name: &str, path: &PathBuf) -> Result<String, anyhow::Error> {
	let now: DateTime<Local> = Local::now();
	let filename = format!(
		"{}-{}-{}-{}-{}:{}:{}.wav",
		name,
		now.year(),
		now.month(),
		now.day(),
		now.hour(),
		now.minute(),
		now.second(),
	);
	Ok(path.join(filename).to_str().unwrap().to_string())
}

/// # Recorder
///
/// The `Recorder` struct is used to record audio.
impl Recorder {

	/// Initializes a new recorder.
	pub fn init(
		name: String,
		path: PathBuf,
		host: HostId,
		sample_rate: u32,
		channels: u16,
		buffer_size: u32,
	) -> Result<Self, anyhow::Error> {

		// Create interrupt handles to be used by the stream or batch loop.
		let interrupt_handles = InterruptHandles::new()?;

		// Select requested host.
		let host = get_host(host)?;

		// Set up the input device and stream with the default input config.
		let device = get_device(host)?;

		// Get default config for the device.
		let default_config = get_default_config(&device)?;

		// Override certain fields of the default stream config with the user's config.
		let user_config = get_user_config(sample_rate, channels, buffer_size)?;

		// Get the hound WAV spec for the user's config.
		let spec = get_spec(&default_config, &user_config)?;

		Ok(Self {
			writer: Arc::new(Mutex::new(None)),
			interrupt_handles,
			default_config,
			user_config,
			device,
			spec,
			name,
			path,
			current_file: "".to_string(),
		})
	}

	fn init_writer(&mut self) -> Result<(), anyhow::Error> {
		let filename = get_filename(&self.name, &self.path)?;
		self.current_file = filename.clone();
		*self.writer.lock().unwrap() = Some(hound::WavWriter::create(filename, self.spec)?);
		Ok(())
	}

	fn create_stream(&self) -> Result<Stream, anyhow::Error> {
		let writer = self.writer.clone();
		let config = self.user_config.clone();
		let err_fn = |err| { eprintln!("An error occurred on stream: {}", err); };

		let stream = match self.default_config.sample_format() {
			cpal::SampleFormat::F32 => self.device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<f32, f32>(data, &writer),
				err_fn,
			)?,
			cpal::SampleFormat::I16 => self.device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<i16, i16>(data, &writer),
				err_fn,
			)?,
			cpal::SampleFormat::U16 => self.device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<u16, i16>(data, &writer),
				err_fn,
			)?,
		};
		Ok(stream)
	}

	pub fn record(&mut self) -> Result<(), anyhow::Error> {
		self.init_writer()?;
		let stream = self.create_stream()?;
		stream.play()?;
		println!("REC: {}", self.current_file);
		self.interrupt_handles.stream_wait();
		drop(stream);
		self.writer.lock().unwrap().take().unwrap().finalize()?;
		println!("STOP: {}", self.current_file);
		Ok(())
	}

	pub fn record_secs(&mut self, secs: u64) -> Result<(), anyhow::Error> {
		self.init_writer()?;
		let stream = self.create_stream()?;
		stream.play()?;
		println!("REC: {}", self.current_file);
		let now = std::time::Instant::now();
		loop {
			std::thread::sleep(std::time::Duration::from_millis(500));
			if now.elapsed().as_secs() >= secs {
				break;
			}
		}
		drop(stream);
		self.writer.lock().unwrap().take().unwrap().finalize()?;
		println!("STOP: {}", self.current_file);
		Ok(())
	}
}

fn write_input_data<T, U>(input: &[T], writer: &WriteHandle)
where
    T: cpal::Sample,
    U: cpal::Sample + hound::Sample,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = cpal::Sample::from(&sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}

pub fn batch_recording(rec: &mut Recorder, secs: u64) -> Result<(), anyhow::Error> {
	while rec.interrupt_handles.batch_is_running() {
		rec.record_secs(secs)?;
	}
	Ok(())
}

pub fn contiguous_recording(rec: &mut Recorder) -> Result<(), anyhow::Error> {
	rec.record()?;
	Ok(())
}

type StreamInterrupt = Arc<(Mutex<bool>, Condvar)>;
type BatchInterrupt= Arc<AtomicBool>;

/// # Interrupts Handling
///
/// The `Recorder` struct has two interrupt mechanisms:
///
/// 1. `stream_interrupt` is used to interrupt the stream when the user presses `ctrl+c`.
/// 2. `batch_interrupt` is used to interrupt the batch recording when the user presses `ctrl+c`.
#[derive(Clone)]
pub struct InterruptHandles {
	batch_interrupt: BatchInterrupt,
	stream_interrupt: StreamInterrupt,
}

impl InterruptHandles {
	pub fn new() -> Result<Self, anyhow::Error> {
		let stream_interrupt = Arc::new((Mutex::new(false), Condvar::new()));
		let stream_interrupt_cloned = stream_interrupt.clone();

		let batch_interrupt = Arc::new(AtomicBool::new(false));
		let batch_interrupt_cloned = batch_interrupt.clone();

		ctrlc::set_handler(move || {
			// Set batch interrupt to true
			batch_interrupt_cloned.store(true, Ordering::SeqCst);

			// Release the stream
			let &(ref lock, ref cvar) = &*stream_interrupt_cloned;
			let mut started = lock.lock().unwrap();
			*started = true;
			cvar.notify_one();
		})?;
		Ok(Self {
			batch_interrupt,
			stream_interrupt,
		})
	}

	pub fn stream_wait(&self) {
		let &(ref lock, ref cvar) = &*self.stream_interrupt;
		let mut started = lock.lock().unwrap();
		while !*started {
			started = cvar.wait(started).unwrap();
		}
	}

	pub fn batch_is_running(&self) -> bool {
		!self.batch_interrupt.load(Ordering::SeqCst)
	}
}