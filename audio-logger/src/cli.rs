use clap::{Parser, ValueEnum};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Hosts {
	Alsa,
	Jack,
}

#[derive(Parser, Debug)]
#[clap(about = "A tool to record audio.")]
pub struct Args {

	/// Filename will be `[NAME]-yyyy-mm-dd-H:M:S.wav`
	#[clap(required = true, short, long)]
	pub name: String,

	/// Path to save the file(s)
	#[clap(long, short, value_parser, value_name = "PATH", value_hint = clap::ValueHint::DirPath)]
    pub output: Option<std::path::PathBuf>,

	/// (optional) Will record in [SECONDS] batches
	#[clap(short, long, value_name = "SECONDS")]
	pub batch_recording: Option<u64>,

	/// Output the available devices and their configurations
	#[clap(long)]
	pub print_configs: bool,

	/// Host API to use
	#[clap(value_enum)]
	pub host: Hosts,

	/// Sample rate in Hz (default = 44,000Hz)
	#[clap(long)]
	pub sample_rate: Option<u32>,

	/// Channels to record
	#[clap(long, value_name = "CHANNELS")]
	pub channels: Option<u16>,

	/// Buffer size in frames
	#[clap(long, value_name = "FRAMES")]
	pub buffer_size: Option<u32>,
}
