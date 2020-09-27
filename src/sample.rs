
use std::path::PathBuf;
use crate::FileMover;

pub fn sample(
    mover: &dyn FileMover,
		pattern: &glob::Pattern,
		source: &str,
    nth: u32,
) -> Result<(), Box<dyn std::error::Error>> {
	let source_directory = PathBuf::from(source);
	let destination_directory = PathBuf::from("./sampled");

	let mut files: Vec<_> = std::fs::read_dir(&source_directory)?
			.into_iter()
			.flatten()
			.filter(|f| f.file_type().unwrap().is_file())
			.map(|f| f.path())
			.filter(|f| pattern.matches(&f.file_name().unwrap_or_default().to_string_lossy()))
			.collect();

	files.sort();

	let results: Result<Vec<_>, _> = files.iter()
		.flat_map(|f| f.file_name())
		.map(|f| PathBuf::from(f))
		.map(move |f| mover.copy(f.as_path(), destination_directory.as_path()))
		.collect();

	results?;
  Ok(())
}
