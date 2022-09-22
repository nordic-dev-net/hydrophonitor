use std::sync::{Arc, Mutex};
use super::*;

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