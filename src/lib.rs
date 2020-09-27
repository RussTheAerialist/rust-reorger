use std::path::{Path, PathBuf};

mod sample;
mod split;

pub use sample::sample;
pub use split::{split, unsplit};

#[derive(Debug)]
pub enum ReorgError {
    NoFilesReturned,
}

impl std::error::Error for ReorgError {}
impl std::fmt::Display for ReorgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ReorgError::*;

        write!(
            f,
            "{}",
            match self {
                NoFilesReturned => "No Files Found",
            }
        )
    }
}

pub trait FileMover {
		fn relocate(&self, source_file: &Path, destination_directory: &Path) -> std::io::Result<()>;
		fn copy(&self, source: &Path, destination: &Path) -> std::io::Result<()>;
    fn remove(&self, p: &Path) -> std::io::Result<()>;
}

fn get_destination_path(source: &Path, destination: &Path) -> std::io::Result<PathBuf> {
    let base_filename = source
        .file_name()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Unable to get filename"))?;
    Ok(PathBuf::from(destination).join(base_filename))
}

pub struct DryRunFileMover {}
impl FileMover for DryRunFileMover {
    fn relocate(&self, source_file: &Path, destination: &Path) -> std::io::Result<()> {
        let destination = get_destination_path(source_file, destination)?;
        println!("{} -> {}", source_file.display(), destination.display());
        Ok(())
    }

    fn remove(&self, _: &Path) -> std::io::Result<()> {
        Ok(()) // This is a no-op for dry runs
    }

    fn copy(&self, source: &Path, destination: &Path) -> std::io::Result<()> {
			let destination = get_destination_path(source, destination)?;
			println!("{} -> {}", source.display(), destination.display());
			Ok(())
    }
}

pub struct OsFileMover {}
impl OsFileMover {
	fn parent<'a>(&self, p: &'a Path) -> std::io::Result<&'a Path> {
		let parent = p.parent().ok_or_else(|| {
			std::io::Error::new(std::io::ErrorKind::Other, "Unable to get filename")
		})?;
		std::fs::create_dir_all(parent)?;
		Ok(parent)
	}
}
impl FileMover for OsFileMover {
    fn relocate(&self, source_file: &Path, destination: &Path) -> std::io::Result<()> {
        let destination = get_destination_path(source_file, destination)?;
        self.parent(&destination)?;

				std::fs::rename(source_file, &destination)?;
        println!("{} -> {}", source_file.display(), destination.display());
        Ok(())
    }

    fn remove(&self, p: &Path) -> std::io::Result<()> {
        std::fs::remove_dir(p)
    }

    fn copy(&self, source: &Path, destination: &Path) -> std::io::Result<()> {
			let destination = get_destination_path(source, destination)?;
			self.parent(&destination)?;
			println!("{} -> {}", source.display(), destination.display());
			std::fs::copy(source, &destination)?;
			Ok(())
    }
}
