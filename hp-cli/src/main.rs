use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
#[clap(about = "A tool to record audio on Linux using the command line.")]
pub enum Commands {
    Install(Install),
    Import(Import),
}

#[derive(Parser, Debug)]
#[clap(about =	"Install hydrophonitor on an SD card.
                You will need to specify the path to the SD card.
                You can find the path to the SD card by running
                `lsblk` in the terminal.")]
pub struct Install {
    /// Path to the SD card. You can find the path to the SD card by running
    /// `lsblk` in the terminal.
    #[clap(short, long, required = true)]
    pub sd_card: PathBuf,
}

#[derive(Parser, Debug)]
#[clap(about =	"Import audio from an SD card.
                You will need to specify the path to the SD card.
                You can find the path to the SD card by running
                `lsblk` in the terminal.
")]
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

const DATA_FOLDER: &str = "/home/pi/data";

fn import_from_sd(sd_card: &PathBuf, output_folder: Option<PathBuf>) {
    let output_folder = match output_folder {
        Some(output_folder) => output_folder,
        None => std::env::current_dir().unwrap(),
    };

    let data_folder = sd_card.join(DATA_FOLDER);

    let mut files = std::fs::read_dir(data_folder).unwrap();

    while let Some(file) = files.next() {
        let file = file.unwrap();
        let file_path = file.path();

        if file_path.is_file() {
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let output_file_path = output_folder.join(file_name);

            std::fs::copy(file_path, output_file_path).unwrap();
        }
    }
}

fn main() {
    let commands = Cli::parse();

    match commands.commands {
        Commands::Install(install) => {
            println!("Installing hydrophonitor on SD card at {:?}", install.sd_card);
        }
        Commands::Import(import) => {
            println!("Importing audio from SD card at {:?}", import.sd_card);

            if let Some(output_folder) = import.output_folder {
                println!("Output folder: {:?}", output_folder);
                import_from_sd(&import.sd_card, Some(output_folder));
            } else {
                println!("Output folder: current directory");
                import_from_sd(&import.sd_card, None);
            }
        }
    }
}
