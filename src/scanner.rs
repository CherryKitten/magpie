use std::{fs, io};
use std::path::{Path, PathBuf};
use lofty::{LoftyError, read_from_path, TaggedFileExt};
use crate::metadata::Track;

pub fn traverse_dir(dir: &Path) -> io::Result<Vec<Track>> {
    let mut tracks = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                tracks.append(&mut traverse_dir(&path).unwrap());
            } else {
                match read_file(&path) {
                    Ok(track) => tracks.push(track),
                    Err(_) => continue
                };
            }
        }
    }
    Ok(tracks)
}

fn read_file(path: &Path) -> Result<Track, LoftyError> {
    let file = read_from_path(path);
    return match file {
        Ok(file) => {
            let tag = file.first_tag().unwrap();
            let track = Track::new(tag, PathBuf::from(path));
            Ok(track)
        }
        Err(e) => Err(e)
    };
}
