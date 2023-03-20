use super::*;
use cpal::{
	StreamConfig,
	SupportedStreamConfig,
	Device,
	HostId,
	Stream,
	traits::{DeviceTrait, StreamTrait},
};
use std::{
	fs::File,
	io::BufWriter,
	path::{PathBuf, Path},
	sync::{Arc, Mutex},
};
use anyhow::Error;

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
	max_seconds: Option<u64>,
}

/// # Recorder
///
/// The `Recorder` struct is used to record audio.
///
/// Use `init()` to initialize the recorder, `record()` to start a continuous recording,
/// and `rec_secs()` to record for a given number of seconds. The Recorder does not
/// need to be reinitialized after a recording is stopped. Calling `record()` or
/// `rec_secs()` again will start a new recording with a new filename according to
/// the time and date.
impl Recorder {

	/// # Init
	///
	/// Initializes the recorder with the given host, sample rate, channel count, and buffer size.
	pub fn init(
		name: String,
		path: PathBuf,
		host: HostId,
		sample_rate: u32,
		channels: u16,
		buffer_size: u32,
		max_seconds: Option<u64>,
	) -> Result<Self, Error> {

		// Create interrupt handles to be used by the stream or batch loop.
		let interrupt_handles = InterruptHandles::new(max_seconds)?;

		// Select requested host.
		let host = get_host(host)?;

		// Set up the input device and stream with the default input config.
		let device = get_device(host)?;

		// Get default config for the device.
		let default_config = get_default_config(&device)?;

		// Override certain fields of the default stream config with the user's config.
		let user_config = get_user_config(sample_rate, channels, buffer_size)?;

		// Get the hound WAV spec for the user's config.
		let spec = get_wav_spec(&default_config, &user_config)?;

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
			max_seconds,
		})
	}

	fn init_writer(&mut self) -> Result<(), Error> {
		let filename = get_filename(&self.name, &self.path);
		self.current_file = filename.clone();
		self.writer = Arc::new(Mutex::new(Some(hound::WavWriter::create(filename, self.spec)?)));
		Ok(())
	}

	fn create_stream(&self) -> Result<Stream, Error> {
		let writer = self.writer.clone();
		let config = self.user_config.clone();
		let err_fn = |err| { eprintln!("{}: An error occurred on stream: {}", get_date_time_string(), err); };

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

	/// # Record
	///
	/// Start a continuous recording. The recording will be stopped when the
	/// user presses `Ctrl+C`.
	pub fn record(&mut self) -> Result<(), Error> {
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

	/// # Record Seconds
	///
	/// Record for a given number of seconds or until the user presses `Ctrl+C`.
	/// Current batch is finished before stopping.
	pub fn record_secs(&mut self, secs: u64) -> Result<(), Error> {
		self.init_writer()?;
		let stream = self.create_stream()?;
		stream.play()?;
        let date_time = get_date_time_string();
        println!("REC[{}]: {}", date_time, self.current_file);
		let now = std::time::Instant::now();
		while self.interrupt_handles.batch_is_running() {
			std::thread::sleep(std::time::Duration::from_millis(1));
			if now.elapsed().as_secs() >= secs {
				break;
			}
		}
		drop(stream);
		let writer = self.writer.clone();
		let current_file = self.current_file.clone();
		std::thread::spawn(move || {
			writer.lock().unwrap().take().unwrap().finalize().unwrap();
            // Print STOP and the time and date
            let date_time = get_date_time_string();
            println!("STOP[{}]: {}", date_time, current_file);
		});
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

fn batch_recording(rec: &mut Recorder, secs: u64) -> Result<(), Error> {
	let now = std::time::Instant::now();
	while rec.interrupt_handles.batch_is_running() {
		match rec.max_seconds {
			Some(max_secs) => {
				if now.elapsed().as_secs() >= max_secs {
					break;
				}
			}
			None => {}
		}
		rec.record_secs(secs)?;
	}
	Ok(())
}

fn continuous_recording(rec: &mut Recorder) -> Result<(), Error> {
	rec.record()?;
	Ok(())
}

#[cfg(target_os = "linux")]
fn match_host_platform(host: Hosts) -> Result<cpal::HostId, Error> {
	match host {
		Hosts::Alsa => Ok(cpal::HostId::Alsa),
		Hosts::Jack => Ok(cpal::HostId::Jack),
		_ => Err(anyhow!("Host not supported on Linux.")),
	}
}

#[cfg(target_os = "macos")]
fn match_host_platform(host: Hosts) -> Result<cpal::HostId, Error> {
	match host {
		Hosts::CoreAudio => Ok(cpal::HostId::CoreAudio),
		_ => Err(anyhow!("Host not supported on macOS.")),
	}
}

#[cfg(target_os = "windows")]
fn match_host_platform(host: Hosts) -> Result<cpal::HostId, Error> {
	match host {
		Hosts::Asio => Ok(cpal::HostId::Asio),
		_ => Err(anyhow!("Host not supported on Windows.")),
	}
}

pub fn record(args: &Rec) -> Result<(), Error> {
	let mut recorder = Recorder::init(
		args.name.clone(),
		match args.output.clone() {
			Some(path) => path,
			None => Path::new("./").to_path_buf(),
		},
		match_host_platform(args.host)?,
		args.sample_rate.unwrap_or(DEFAULT_SAMPLE_RATE),
		args.channels.unwrap_or(DEFAULT_CHANNEL_COUNT),
		args.buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE),
		args.max_seconds,
	)?;

	match args.batch_recording {
		Some(seconds) => batch_recording(&mut recorder, seconds),
		None => continuous_recording(&mut recorder),
	}
}
