use crate::db::models::Track;
use crate::db::DbPool;
use anyhow::Result;
use lofty::{read_from_path, Accessor, AudioFile, FileProperties, Tag, TaggedFileExt};
use log::{error, info, trace};
use std::fs;
use std::path::Path;

pub fn scan(dir: &Path, pool: DbPool) -> Result<()> {
    info!("Scanning directory {:?}", dir);

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        let mut conn = pool.get()?;

        if path.is_dir() {
            if let Err(e) = scan(&path, pool.clone()) {
                error!("{e}");
                continue;
            }
        } else {
            match read_file(&path)? {
                FileType::Music(tag) => {
                    info!("Found track {:?}", tag.0.title());
                    if !Track::check(&path, &mut conn) {
                        Track::new(tag, &path, &mut conn)?;
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
        "flac" | "mp3" | "opus" => {
            if let Ok(file) = read_tags(path) {
                FileType::Music(file)
            } else {
                FileType::Unsupported
            }
        }
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
