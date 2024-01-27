use color_eyre::{eyre::OptionExt, Result};
use lofty::{read_from_path, Accessor, TaggedFileExt};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

#[derive(Debug, Default, Clone)]
pub struct Scan {
    root: PathBuf,
    tracks: Vec<PathBuf>,
    images: Vec<PathBuf>,
    artists: Vec<PathBuf>,
    albums: Vec<PathBuf>,
    other: Vec<PathBuf>,
}

pub fn scan_dir(dir: &Path) -> Result<Scan> {
    let mut scan_result = Scan::default();
    tracing::info!("Scanning directory {:?}", dir);
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| {
        e.ok().filter(|e| {
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
    }) {
        if entry.metadata()?.is_dir() {
            match entry.depth() {
                0 => scan_result.root = entry.into_path(),
                1 => scan_result.artists.push(entry.into_path()),
                2 => scan_result.albums.push(entry.into_path()),
                _ => scan_result.other.push(entry.into_path()),
            }
        } else {
            match entry
                .path()
                .extension()
                .unwrap_or_default()
                .to_ascii_lowercase()
                .to_str()
                .unwrap_or_default()
            {
                "flac" | "mp3" | "opus" | "aif" | "aiff" | "wav" | "alac" | "ape" | "m4a"
                | "ogg" | "aac" => scan_result.tracks.push(entry.into_path()),
                "png" | "jpg" | "jpeg" => scan_result.images.push(entry.into_path()),
                _ => scan_result.other.push(entry.into_path()),
            }
        }
    }

    Ok(scan_result)
}

pub fn scan_library() -> Result<()> {
    let path = PathBuf::from("/home/sammy/magpie_testing/library/");

    let scan_result = scan_dir(&path).unwrap();

    for track in scan_result.tracks {
        let tagged_file = read_from_path(track)?;

        let tag = if let Some(tag) = tagged_file.primary_tag() {
            tag
        } else if let Some(tag) = tagged_file.first_tag() {
            tag
        } else {
            continue;
        };

        let title = tag.title();
        let artist = tag.artist();
        let album = tag.album();

        println!("{artist:?} - {album:?} - {title:?}");
    }

    Ok(())
}
