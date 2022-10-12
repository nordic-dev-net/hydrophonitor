use hound::{WavReader, WavWriter};
use anyhow::{Result, Error};

pub fn merge_wavs(input: &std::path::PathBuf, output: &std::path::PathBuf) -> Result<(), Error> {
	// Read files from input directory
	let mut files = std::fs::read_dir(input)?
		.filter_map(|entry| entry.ok())
		.filter(|entry| entry.file_type().ok().map(|t| t.is_file()).unwrap_or(false))
		.filter(|entry| entry.path().extension().unwrap_or_default() == "wav")
		.collect::<Vec<_>>();

	// Sort files by name
	files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

	// Use the name of the first file as the name of the output file
	let output_name = files.first().unwrap().file_name();
	let output_name = output_name.to_str().unwrap();

	// Get wav spec from file
	let spec = WavReader::open(files.first().unwrap().path())?.spec();
	let mut writer = WavWriter::create(output.join(output_name), spec)?;

	match spec.sample_format {
		hound::SampleFormat::Float => {
			for file in files {
				let mut reader = WavReader::open(file.path())?;
				for sample in reader.samples::<f32>() {
					writer.write_sample(sample?)?;
				}
			}
		},
		hound::SampleFormat::Int => {
			for file in files {
				let mut reader = WavReader::open(file.path())?;
				for sample in reader.samples::<i32>() {
					writer.write_sample(sample?)?;
				}
			}
		},
	}
	writer.finalize()?;
	Ok(())
}
