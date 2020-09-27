use std::path::{Path, PathBuf};
use glob::Pattern;

#[derive(Debug)]
pub enum ReorgError {
	NoFilesReturned
}

impl std::error::Error for ReorgError { }
impl std::fmt::Display for ReorgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			use ReorgError::*;

			write!(f, "{}", match self {
				NoFilesReturned => "No Files Found",
			})
    }
}

pub trait FileMover {
	fn relocate(&self, source_file: &Path, destination_directory: &Path) -> std::io::Result<()>;
}

fn get_destination_path(source: &Path, destination: &Path) -> std::io::Result<PathBuf> {
	let base_filename = source.file_name()
																		.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Unable to get filename"))?;
  Ok(PathBuf::from(destination).join(base_filename))
}

pub struct DryRunFileMover { }
impl FileMover for DryRunFileMover {
	fn relocate(&self, source_file: &Path, destination: &Path) -> std::io::Result<()> {
		let destination = get_destination_path(source_file, destination)?;
		println!("{} -> {}", source_file.display(), destination.display());
		Ok(())
	}
}

pub struct OsFileMover { }
impl FileMover for OsFileMover {
	fn relocate(&self, source_file: &Path, destination: &Path) -> std::io::Result<()> {
		let destination = get_destination_path(source_file, destination)?;
		let parent = destination.parent().ok_or_else(||  std::io::Error::new(std::io::ErrorKind::Other, "Unable to get filename"))?;
		std::fs::create_dir_all(parent)?;
		std::fs::rename(source_file, &destination)?;
		println!("{} -> {}", source_file.display(), destination.display());
		Ok(())
	}
}

pub fn split(mover: &Box<dyn FileMover>, pattern: &glob::Pattern) -> Result<(), Box<dyn std::error::Error>> {
	let current_directory = std::env::current_dir()?;

	let files : Vec<_> = std::fs::read_dir(&current_directory)?
													.into_iter()
													.flatten()
													.filter(|f| f.file_type().unwrap().is_file())
													.map(|f| f.path())
													.filter(|f| pattern.matches(&f.file_name().unwrap_or_default().to_string_lossy()))
													.collect();
	let num_files = files.len();
	if num_files == 0 {
		return Err(ReorgError::NoFilesReturned.into());
	}
	let groupings = files.chunks(1000);
	let results: Result<Vec<_>, _> = groupings.enumerate().flat_map(|(i, b)| {
		let destination_path = current_directory.join(format!("{:04}", i));
		b.iter().map(move |p| mover.as_ref().relocate(p.as_path(), destination_path.as_path()))
	}).collect();

	results?;
	Ok(())
}

pub fn unsplit(mover: &Box<dyn FileMover>, pattern: &glob::Pattern) -> Result<(), Box<dyn std::error::Error>> {
	let current_directory = std::env::current_dir()?;

	let files : Vec<_> = std::fs::read_dir(&current_directory)?
													.into_iter()
													.flatten()
													.flat_map(|f| {
														if f.file_type().unwrap().is_dir() {
															std::fs::read_dir(f.path()).unwrap().into_iter().flatten().collect()
														} else {
															vec![f]
														}
													})
													.filter(|f| f.file_type().unwrap().is_file())
													.map(|f| f.path())
													.filter(|f| pattern.matches(&f.file_name().unwrap_or_default().to_string_lossy()))
													.collect();

	let results: Result<Vec<_>, _> = files.iter().map(move |f| mover.as_ref().relocate(f, current_directory.as_path())).collect();

	results?;
	Ok(())
}