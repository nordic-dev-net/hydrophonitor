//! # Main
mod cli;
mod recorder;
mod print_configs;
mod getters;
mod input_handling;
mod merge;
mod constants;
mod play;

use print_configs::*;
use cli::*;
use merge::*;
use recorder::*;
use constants::*;
use getters::*;
use input_handling::*;
use play::*;
use anyhow::{Error, Result, anyhow};

use clap::Parser;

fn main() -> Result<(), Error> {
	let cli = Cli::parse();

	match &cli.command {
		Commands::Rec(args) => {
			record(&args)?;
		},
		Commands::Play(args) => {
			play(&args.input)?;
		},
		Commands::Info(args) => {
			if args.print_configs {
				print_configs()?;
			}
		},
		Commands::Merge(args) => {
			merge_wavs(&args.input, &args.output)?;
		}
	}
	Ok(())
}
