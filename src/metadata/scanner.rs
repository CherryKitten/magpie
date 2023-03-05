use crate::db::models::Track;
use anyhow::Result;
use lofty::{read_from_path, Accessor, Tag, TaggedFileExt, AudioFile, FileProperties};
use log::{info, trace};
use std::fs;
use std::path::Path;

pub fn scan(dir: &Path) -> Result<()> {
    info!("Scanning directory {:?}", dir);

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_dir() {
            scan(&path)?;
        } else {
            match read_file(&path)? {
                FileType::Music(tag) => {
                    info!("Found track {:?}", tag.0.title());
                    if !Track::check(&path) {
                        Track::new(tag, &path)?;
                    };
                    continue;
                }
                FileType::Image => {
                    info!("Found image {:?}", path);
                }
                FileType::Unsupported => {
                    info!("Unsupported filetype on {:?}", path);
                    continue;
                }
            }
        }
    }

    Ok(())
}

enum FileType {
    Music((Tag, FileProperties)),
    Image,
    Unsupported,
}

fn read_file(path: &Path) -> Result<FileType> {
    trace!("Reading file {:?}", path);

    let file = match path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
    {
        "flac" | "mp3" | "opus" => FileType::Music(read_tags(path)?),
        "png" | "jpg" | "jpeg" | "webp" => FileType::Image,
        _ => FileType::Unsupported,
    };

    Ok(file)
}

fn read_tags(path: &Path) -> Result<(Tag, FileProperties)> {
    let file = read_from_path(path)?;

    let tag = match file.primary_tag() {
        Some(tag) => tag,
        None => match file.first_tag() {
            Some(tag) => tag,
            None => return Err(anyhow::Error::msg("Could not find tag in file")),
        },
    };

    let properties = file.properties();

    Ok((tag.to_owned(), properties.to_owned()))
}
