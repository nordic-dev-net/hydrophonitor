//! Records a WAV file (roughly 3 seconds long) using the default input device and config.
//!
//! The input data is recorded to "$CARGO_MANIFEST_DIR/recorded.wav".

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::{Arc, Mutex};
use chrono::prelude::*;

fn main() -> Result<(), anyhow::Error> {
	// Use jack host, requires the jack server to be running
    let host = cpal::host_from_id(cpal::available_hosts()
		.into_iter()
		.find(|id| *id == cpal::HostId::Jack)
		.expect(
			"make sure feature jack is specified for cpal. only works on OSes where jack is available",
		)).expect("jack host unavailable");

    // Set up the input device and stream with the default input config.
    let device = host.default_input_device()
	    .expect("failed to find input device");

    println!("Input device: {}", device.name()?);

    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

	// Location where files will be outputted
	let path = Path::new("/home/shared/logger-raspi-setup/data/audio/");
    let spec = wav_spec_from_config(&config);

	for _ in 0..5 {
		// The WAV file we're recording to.
		let ts: String = Utc::now().format("%Y-%m-%dT%H-%M-%S.%f").to_string();
		let file: String = path.to_str().unwrap().to_owned() + &ts + "_audio_data.wav";

		let writer = hound::WavWriter::create(file.clone(), spec)?;
		let writer = Arc::new(Mutex::new(Some(writer)));
	
		// Run the input stream on a separate thread.
		let writer_2 = writer.clone();
		
		let config_2 = config.clone();
		
		let err_fn = move |err| {
			eprintln!("an error occurred on stream: {}", err);
		};
		
		let stream = match config.sample_format() {
			cpal::SampleFormat::F32 => device.build_input_stream(
				&config_2.into(),
				move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
				err_fn,
			)?,
			cpal::SampleFormat::I16 => device.build_input_stream(
				&config_2.into(),
				move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
				err_fn,
			)?,
			cpal::SampleFormat::U16 => device.build_input_stream(
				&config_2.into(),
				move |data, _: &_| write_input_data::<u16, i16>(data, &writer_2),
				err_fn,
			)?,
		};
		
		// Start recording
		println!("Begin recording at {}", Utc::now());
		stream.play()?;
	
		// Let recording go for roughly five seconds.
		std::thread::sleep(std::time::Duration::from_secs(5));
		drop(stream);
		writer.lock().unwrap().take().unwrap().finalize()?;
		println!("Recording {} complete!", file);
	}
    Ok(())
}

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    match format {
        cpal::SampleFormat::U16 => hound::SampleFormat::Int,
        cpal::SampleFormat::I16 => hound::SampleFormat::Int,
        cpal::SampleFormat::F32 => hound::SampleFormat::Float,
    }
}

fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
    hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: sample_format(config.sample_format()),
    }
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
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
