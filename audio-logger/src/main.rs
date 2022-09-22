//! Records a WAV file (roughly 3 seconds long) using the default input device and config.
//!
//! The input data is recorded to "$CARGO_MANIFEST_DIR/recorded.wav".
mod cli;
mod recorder;
mod print_configs;

use clap::Parser;
use recorder::{batch_recording, contiguous_recording};
use cli::*;
use std::path::Path;
use print_configs::*;
use std::sync::{Arc, Mutex, Condvar, atomic::{AtomicBool, Ordering}};

const DEFAULT_SAMPLE_RATE: u32 = 44100;
const DEFAULT_CHANNEL_COUNT: u16 = 1;
const DEFAULT_BUFFER_SIZE: u32 = 1024;
const ALLOWED_SAMPLE_RATES: &[u32] = &[44100, 48000, 88200, 96000, 176400, 192000];
const MAX_CHANNEL_COUNT: u16 = 2;
const MIN_BUFFER_SIZE: usize = 64;
const MAX_BUFFER_SIZE: usize = 8192;
const FMT_TIME: &str = "%Y-%m-%d-%H:%M:%S.%f";

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

fn main() -> Result<(), anyhow::Error> {
	let args = Args::parse();

	if args.print_configs {
		print_configs()?;
		return Ok(());
	}

	let interrupt_handles = InterruptHandles::new()?;

	match args.batch_recording {
		Some(secs) => {
			batch_recording(&args, secs, interrupt_handles)?;
		},
		None => {
			contiguous_recording(&args, interrupt_handles)?;
		}
	}

	Ok(())
}

