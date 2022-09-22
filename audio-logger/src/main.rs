//! Records a WAV file (roughly 3 seconds long) using the default input device and config.
//!
//! The input data is recorded to "$CARGO_MANIFEST_DIR/recorded.wav".
mod cli;
mod recorder;
mod print_configs;
mod getters;
mod input_handling;

use clap::Parser;
use recorder::{batch_recording, contiguous_recording};
use cli::*;
use std::path::Path;
use print_configs::*;
use std::sync::{Condvar, atomic::{AtomicBool, Ordering}};

const DEFAULT_SAMPLE_RATE: u32 = 44100;
const DEFAULT_CHANNEL_COUNT: u16 = 1;
const DEFAULT_BUFFER_SIZE: u32 = 1024;
const ALLOWED_SAMPLE_RATES: &[u32] = &[44100, 48000, 88200, 96000, 176400, 192000];
const MAX_CHANNEL_COUNT: u16 = 2;
const MIN_BUFFER_SIZE: usize = 64;
const MAX_BUFFER_SIZE: usize = 8192;

fn main() -> Result<(), anyhow::Error> {
	let args = Args::parse();

	if args.print_configs {
		print_configs()?;
		return Ok(());
	}

	let mut recorder = recorder::Recorder::init(
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
	)?;

	match args.batch_recording {
		Some(secs) => {
			batch_recording(&mut recorder, secs)?;
		},
		None => {
			contiguous_recording(&mut recorder)?;
		}
	}

	Ok(())
}

