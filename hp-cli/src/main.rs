use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use indicatif::ProgressBar;
use hound::{WavReader, WavWriter};

const DATA_FOLDER: &str = "home/pi/data";

#[derive(Subcommand)]
#[clap(about = "A tool to record audio on Linux using the command line.")]
pub enum Commands {
    Install(Install),
    Import(Import),
}

#[derive(Parser, Debug)]
#[clap(about =	"Install hydrophonitor on an SD card.")]
pub struct Install {
    /// Path to the SD card. You can find the path to the SD card by running
    /// `lsblk` in the terminal.
    #[clap(short, long, required = true)]
    pub sd_card: PathBuf,
}

#[derive(Parser, Debug)]
#[clap(about =	"Import audio from an SD card.")]
pub struct Import {
    /// Path to the SD card. You can find the path to the SD card by running
    /// `lsblk` in the terminal.
    #[clap(short, long, required = true)]
    pub sd_card: PathBuf,

    /// Path to the output folder. If not specified, the output folder will be
    /// the current directory.
    #[clap(short, long)]
    pub output_folder: Option<PathBuf>,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

fn import_from_sd(sd_card: &mut PathBuf, output_folder: Option<PathBuf>) -> Result<(), Box<dyn Error>>{
    let output_folder = match output_folder {
        Some(output_folder) => output_folder,
        None => std::env::current_dir().unwrap(),
    };

    sd_card.push(DATA_FOLDER);

    let count = WalkDir::new(sd_card.clone()).into_iter().count();
    let progress_bar = ProgressBar::new(count as u64);

    for entry in WalkDir::new(sd_card.clone()) {
        let entry = entry?; 
        let from = entry.path();
        let to = output_folder.join(from.strip_prefix(sd_card.clone())?);

        if entry.file_type().is_dir() {
            fs::create_dir_all(to)?;
        } else if entry.file_type().is_file() {
            fs::copy(from, to)?;
        }
        progress_bar.inc(1);
    }
    progress_bar.finish();
    Ok(())
}


pub fn merge_wavs(input: &std::path::PathBuf, output: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
	// Read files from input directory
	let mut files = std::fs::read_dir(input)?
		.filter_map(|entry| entry.ok())
		.filter(|entry| entry.file_type().ok().map(|t| t.is_file()).unwrap_or(false))
		.filter(|entry| entry.path().extension().unwrap_or_default() == "wav")
		.collect::<Vec<_>>();

	// Sort files by name
	files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

	let output_name = files.first().unwrap().file_name();
	let output_name = output_name.to_str().unwrap();

	// Get wav spec from file
	let spec = WavReader::open(files.first().unwrap().path())?.spec();
	let mut writer = WavWriter::create(output.join(output_name), spec)?;

    let progress_bar = ProgressBar::new(files.len() as u64);
	match spec.sample_format {
		hound::SampleFormat::Float => {
			for file in files {
				let mut reader = WavReader::open(file.path())?;
				for sample in reader.samples::<f32>() {
					writer.write_sample(sample?)?;
				}
                progress_bar.inc(1);
			}
		},
		hound::SampleFormat::Int => {
			for file in files {
				let mut reader = WavReader::open(file.path())?;
				for sample in reader.samples::<i32>() {
					writer.write_sample(sample?)?;
				}
                progress_bar.inc(1);
			}
		},
	}
    progress_bar.finish();
	writer.finalize()?;
	Ok(())
}

fn main() {
    let commands = Cli::parse();

    match commands.commands {
        Commands::Install(install) => {
            println!("Installing hydrophonitor on SD card at {:?}", install.sd_card);
        }
        Commands::Import(mut import) => {
            println!("Importing audio from SD card at {:?}", import.sd_card);

            if let Some(output_folder) = &import.output_folder {
                println!("Output folder: {:?}", output_folder);
                import_from_sd(&mut import.sd_card, Some(output_folder.clone())).unwrap();
            } else {
                println!("Output folder: current directory");
                import_from_sd(&mut import.sd_card, None).unwrap();
            }

            // Iterate folders inside output folder. Inside each iterated folder there is
            // a folder called "audio" which contains the wav files. Merge them into a single
            // wav file and delete the "audio" folder.
            let output_folder = match import.output_folder {
                Some(output_folder) => output_folder,
                None => std::env::current_dir().unwrap(),
            };

            for entry in WalkDir::new(output_folder.clone()) {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    let audio_folder = path.join("audio");
                    if audio_folder.exists() {
                        merge_wavs(&audio_folder, &PathBuf::from(path)).unwrap();
                        fs::remove_dir_all(audio_folder).unwrap();
                    }
                }
            }
        }
    }
}
