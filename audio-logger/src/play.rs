//! Play audio from a .wav file.

use anyhow::{Error, Result};
use rodio::{Decoder, OutputStream, Sink};

pub fn play(path: &std::path::PathBuf) -> Result<(), Error> {
	let file = std::fs::File::open(path)?;
	let source = Decoder::new(file).unwrap();
	let (_stream, stream_handle) = OutputStream::try_default()?;
	let sink = Sink::try_new(&stream_handle)?;
	sink.append(source);
	sink.sleep_until_end();
	Ok(())
}