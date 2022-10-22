use super::*;
use cpal::{
	StreamConfig,
	SupportedStreamConfig,
	Device,
	HostId,
	Host,
	SampleRate,
	BufferSize,
	traits::{DeviceTrait, HostTrait},
};
use hound::WavSpec;
use std::path::PathBuf;
use chrono::*;
use anyhow::{Error, Result, anyhow};

/// # Get Host
///
/// Returns the host with the given id if it's available.
pub fn get_host(host: HostId) -> Result<Host, Error> {
	Ok(cpal::host_from_id(cpal::available_hosts()
		.into_iter()
		.find(|id| *id == host)
		.ok_or(anyhow!("Requested host device not found"))?
	)?)
}

/// # Get Device
///
/// Returns the default input device for the host if it's available.
pub fn get_device(host: Host) -> Result<Device, Error> {
	Ok(host.default_input_device()
		.ok_or(anyhow!("No input device available. Try running `jackd -R -d alsa -d hw:0`",
	))?)
}

/// # Get Default Config
///
/// Get the default config for the given device.
pub fn get_default_config(device: &Device) -> Result<SupportedStreamConfig, Error> {
	Ok(device.default_input_config()?)
}

/// # Get User Config
///
/// Overrides certain fields of the default stream config with the user's config.
///
/// sample_rate: The user's sample rate if it is supported by the device, otherwise the default sample rate.
/// channels: The user's number of channels if it is supported by the device, otherwise the default number of channels.
/// buffer_size: The user's buffer size if it is supported by the device, otherwise the default buffer size.
pub fn get_user_config(sample_rate: u32, channels: u16, buffer_size: u32) -> Result<StreamConfig, Error> {
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

/// # Get WAV Spec
///
/// Get the WAV spec for the given stream config.
pub fn get_wav_spec(default_config: &SupportedStreamConfig, user_config: &StreamConfig) -> Result<WavSpec, Error> {
	Ok(WavSpec {
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

pub fn get_date_time_string() -> String {
	let now: DateTime<Local> = Local::now();
	format!(
		"{:02}-{:02}-{:02}_{:02}:{:02}:{:02}",
		now.year(), now.month(), now.day(),
		now.hour(), now.minute(), now.second(),
	)
}
/// # Get Filename
///
/// Get the filename for the current recording according to the given format,
/// the current date and time, and the name prefix.
pub fn get_filename(name: &str, path: &PathBuf) -> String {
	let mut filename = path.clone();
	filename.push(format!("{}_{}.wav", get_date_time_string(), name));
	filename.to_str().unwrap().to_string()
}
