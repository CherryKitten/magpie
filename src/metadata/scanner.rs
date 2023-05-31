use std::future::Future;
use std::path::Path;
use std::pin::Pin;

use anyhow::Result;
use lofty::{read_from_path, AudioFile, FileProperties, Tag, TaggedFileExt};
use tokio::{fs, spawn};

use crate::db::DbPool;
use crate::metadata::Track;

pub async fn scan(dir: &Path, pool: DbPool) -> Pin<Box<dyn Future<Output = Result<()>>>> {
    log::info!("Scanning directory {:?}", dir);

    let mut entries = fs::read_dir(dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        if entry.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        if entry.metadata().await?.file_type().is_dir() {
            scan(dir, pool.clone()).await;
        } else {
            let path = entry.path();
            let _ = read_file(&*path, pool.clone());
        }
    }

    // for entry in WalkDir::new(dir).into_iter().filter_map(|e| {
    //     e.ok().filter(|e| {
    //         !e.file_name()
    //             .to_str()
    //             .map(|s| s.starts_with('.'))
    //             .unwrap_or(false)
    //     })
    // }) {
    //     let path = entry.path();
    //     let _ = read_file(path, pool.clone());
    // }

    Ok(())
}

fn read_file(path: &Path, pool: DbPool) -> Result<()> {
    log::debug!("Reading file {:?}", path);

    match path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_lowercase()
        .as_str()
    {
        "flac" | "mp3" | "opus" | "aif" | "aiff" | "wav" | "alac" | "ape" | "m4a" | "ogg" => {
            if let Ok(file) = read_tags(path) {
                let mut conn = pool.get()?;
                if !Track::check(path, &mut conn) {
                    Track::new(file, path, &mut conn)?;
                }
            }
        }
        // TODO: Do something with image files
        //"png" | "jpg" | "jpeg" | "webp" => {}
        _ => {}
    };

    Ok(())
}

fn read_tags(path: &Path) -> Result<(Tag, FileProperties)> {
    let file = read_from_path(path)?;

    let tag = file.primary_tag().or_else(|| file.first_tag());

    match tag {
        Some(tag) => Ok((tag.to_owned(), file.properties().to_owned())),
        None => Err(anyhow::Error::msg("Could not find tag in file")),
    }
}
