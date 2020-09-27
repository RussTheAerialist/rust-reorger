use crate::{FileMover, ReorgError};
use std::collections::HashSet;

pub fn split(
    mover: &dyn FileMover,
    pattern: &glob::Pattern,
) -> Result<(), Box<dyn std::error::Error>> {
    let current_directory = std::env::current_dir()?;

    let files: Vec<_> = std::fs::read_dir(&current_directory)?
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
    let results: Result<Vec<_>, _> = groupings
        .enumerate()
        .flat_map(|(i, b)| {
            let destination_path = current_directory.join(format!("{:04}", i));
            b.iter().map(move |p| {
                mover
                    .relocate(p.as_path(), destination_path.as_path())
            })
        })
        .collect();

    results?;
    Ok(())
}

pub fn unsplit(
    mover: &dyn FileMover,
    pattern: &glob::Pattern,
) -> Result<(), Box<dyn std::error::Error>> {
    let current_directory = std::env::current_dir()?;

    let files: Vec<_> = std::fs::read_dir(&current_directory)?
        .into_iter()
        .flatten()
        .flat_map(|f| {
            if f.file_type().unwrap().is_dir() {
                std::fs::read_dir(f.path())
                    .unwrap()
                    .into_iter()
                    .flatten()
                    .collect()
            } else {
                vec![f]
            }
        })
        .filter(|f| f.file_type().unwrap().is_file())
        .map(|f| f.path())
        .filter(|f| pattern.matches(&f.file_name().unwrap_or_default().to_string_lossy()))
        .collect();

    let results: Result<Vec<_>, _> = files
        .iter()
        .map(move |f| mover.relocate(f, current_directory.as_path()))
        .collect();
    results?;
    let directories_to_clean: HashSet<_> = files.iter().flat_map(|f| f.parent()).collect();
    directories_to_clean.iter().for_each(|f| {
        mover.remove(f).unwrap_or(());
    });
    Ok(())
}
