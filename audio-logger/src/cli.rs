use clap::{Parser, ValueEnum, Subcommand};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Hosts {
	Alsa,
	Jack,
	CoreAudio,
	Asio,
}

#[derive(Subcommand)]
#[clap(about = "A tool to record audio on Linux using the command line.")]
pub enum Commands {
    /// Record audio either continously or in batches (in case a possible
	/// interruption is an issue)
	Rec(Rec),
	/// Play audio from a .wav file
	Play(Play),
	/// Merge audio resulting from a batch recording into a single file
	Merge(Merge),
	/// Get reports of system's audio resources
	Info(Info),
}

#[derive(Parser, Debug)]
#[clap(about =	"Merge audio resulting from a batch recording into a single
				file.
")]
pub struct Merge {
	/// The directory to look for files to merge.
	#[clap(short, long, default_value = "./", required = true)]
	pub input: std::path::PathBuf,

	/// The directory to save the merged file to.
	#[clap(short, long, default_value = "./")]
	pub output: std::path::PathBuf,
}

#[derive(Parser, Debug)]
#[clap(about =	"Record audio either continously or in batches (in case a
				possible interruption is an issue).
")]
pub struct Rec {

	/// Filename will be `[NAME]-yyyy-mm-dd-H:M:S.wav`
	#[clap(required = true, short, long)]
	pub name: String,

	/// Path to save the file(s)
	#[clap(long, short, value_parser, value_name = "PATH", value_hint = clap::ValueHint::DirPath)]
    pub output: Option<std::path::PathBuf>,

	/// (optional) Will record in [SECONDS] batches
	#[clap(short, long, value_name = "SECONDS")]
	pub batch_recording: Option<u64>,

	/// (optional) Will record for a total of [SECONDS]
	#[clap(short, long, value_name = "MAX_SECONDS")]
	pub max_seconds: Option<u64>,

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
#[derive(Parser, Debug)]
#[clap(about =	"Play audio from a .wav file.
")]
pub struct Play {
	/// Path to the file to play
	#[clap(value_parser, value_name = "PATH", value_hint = clap::ValueHint::FilePath)]
	pub input: std::path::PathBuf,
}

#[derive(Parser, Debug)]
#[clap(about =	"Get reports of system's audio resources.")]
pub struct Info {
	/// Output the available devices and their configurations
	#[clap(long)]
	pub print_configs: bool,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}