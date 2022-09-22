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
	interrupt: InterruptHandles,
	default_config: SupportedStreamConfig,
	user_config: StreamConfig,
	device: Device,
	filename: String,
}

/// # Stream User Config
///
/// Overrides certain fields of the default stream config with the user's config.
///
/// sample_rate: The user's sample rate if it is supported by the device, otherwise the default sample rate.
/// channels: The user's number of channels if it is supported by the device, otherwise the default number of channels.
/// buffer_size: The user's buffer size if it is supported by the device, otherwise the default buffer size.
fn stream_user_config(sample_rate: u32, channels: u16, buffer_size: u32) -> Result<StreamConfig, anyhow::Error> {
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
		interrupt: InterruptHandles,
	) -> Result<Self, anyhow::Error> {

		// Select requested host
		let host = cpal::host_from_id(cpal::available_hosts()
			.into_iter()
			.find(|id| *id == host)
			.ok_or(anyhow!("Requested host device not found"))?
		)?;

		// Set up the input device and stream with the default input config.
		let device = host.default_input_device()
			.ok_or(anyhow!("No input device available. Try running `jackd -R -d alsa -d hw:0`",
		))?;

		let default_config = device.default_input_config()?;
		let user_config = stream_user_config(sample_rate, channels, buffer_size)?;

		let spec = hound::WavSpec {
			channels: user_config.channels as _,
			sample_rate: user_config.sample_rate.0 as _,
			bits_per_sample: (default_config.sample_format().sample_size() * 8) as _,
			sample_format: match default_config.sample_format() {
				cpal::SampleFormat::U16 => hound::SampleFormat::Int,
				cpal::SampleFormat::I16 => hound::SampleFormat::Int,
				cpal::SampleFormat::F32 => hound::SampleFormat::Float,
			},
		};

		// The WAV file we're recording to.
		let ts: String = Utc::now().format(FMT_TIME).to_string();
		let filename: String = path.to_str().unwrap().to_owned() + &name + "-" + &ts + ".wav";

		Ok(Self {
			writer: Arc::new(Mutex::new(Some(hound::WavWriter::create(filename.clone(), spec)?))),
			interrupt,
			default_config,
			user_config,
			device,
			filename,
		})
	}

	fn create_stream(&self) -> Result<Stream, anyhow::Error> {
		let writer = self.writer.clone();
		let config = self.user_config.clone();
		let err_fn = |err| { eprintln!("an error occurred on stream: {}", err); };

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

	pub fn record(&self) -> Result<(), anyhow::Error> {
		let stream = self.create_stream()?;
		stream.play()?;
		println!("REC: {}", self.filename);
		self.interrupt.stream_wait();
		drop(stream);
		self.writer.lock().unwrap().take().unwrap().finalize()?;
		println!("STOP: {}", self.filename);
		Ok(())
	}

	pub fn record_secs(&self, secs: u64) -> Result<(), anyhow::Error> {
		let stream = self.create_stream()?;
		stream.play()?;
		println!("REC: {}", self.filename);
		let now = std::time::Instant::now();
		loop {
			std::thread::sleep(std::time::Duration::from_millis(500));
			if now.elapsed().as_secs() >= secs {
				break;
			}
		}
		drop(stream);
		self.writer.lock().unwrap().take().unwrap().finalize()?;
		println!("STOP: {}", self.filename);
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

pub fn batch_recording(args: &Args, secs: u64, interrupt_handles: InterruptHandles) -> Result<(), anyhow::Error> {
	while interrupt_handles.batch_is_running() {
		let recorder = recorder::Recorder::init(
			args.name.clone(),
			match args.output.clone() {
				Some(path) => path,
				None => Path::new("./").to_path_buf(),
			},
			match args.host {
				Hosts::Alsa => cpal::HostId::Alsa,
				Hosts::Jack => cpal::HostId::Jack,
			},
			args.sample_rate.unwrap_or(DEFAULT_SAMPLE_RATE),
			args.channels.unwrap_or(DEFAULT_CHANNEL_COUNT),
			args.buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE),
			interrupt_handles.clone(),
		)?;
		recorder.record_secs(secs)?;
	}
	Ok(())
}

pub fn contiguous_recording(args: &Args, interrupt_handles: InterruptHandles) -> Result<(), anyhow::Error> {
	let recorder = recorder::Recorder::init(
		args.name.clone(),
		match args.output.clone() {
			Some(path) => path,
			None => Path::new("./").to_path_buf(),
		},
		match args.host {
			Hosts::Alsa => cpal::HostId::Alsa,
			Hosts::Jack => cpal::HostId::Jack,
		},
		args.sample_rate.unwrap_or(DEFAULT_SAMPLE_RATE),
		args.channels.unwrap_or(DEFAULT_CHANNEL_COUNT),
		args.buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE),
		interrupt_handles.clone(),
	)?;
	recorder.record()?;
	Ok(())
}
