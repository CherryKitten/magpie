use std::{fs, io};
use std::path::Path;
use lofty::{Accessor, read_from_path, Tag, TaggedFileExt, LoftyError, ItemKey};

fn main() -> Result<(), io::Error> {
    let test_path = Path::new("test_data/music");
    let mut tracks = vec![];
    println!("Hello, {}!", test_path.display());
    tracks.append(&mut traverse_dir(test_path).unwrap());
    for track in tracks {
        println!("{:?}", track);
    }
    Ok(())
}

fn traverse_dir(dir: &Path) -> io::Result<Vec<Track>> {
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
            let track = Track::new(tag);
            Ok(track)
        }
        Err(e) => Err(e)
    };
}

#[derive(Debug)]
struct Track {
    artists: Vec<String>,
    album: String,
    albumartists: Vec<String>,
    title: String,
    track_number: Option<u32>,
    disc_number: Option<u32>,
}

impl Track {
    fn new(tag: &Tag) -> Self {
        let artists = tag.get_strings(&ItemKey::TrackArtist);
        let albumartists = tag.get_strings(&ItemKey::AlbumArtist);
        let album = tag.album().unwrap();
        let title = tag.title().unwrap();
        let disc_number = tag.disk();
        let track_number = tag.track();
        Track {
            artists: {
                let mut temp_vec = vec![];
                for artist in artists {
                    temp_vec.push(artist.to_string());
                }
                temp_vec
            },
            albumartists: {
                let mut temp_vec = vec![];
                for artist in albumartists {
                    temp_vec.push(artist.to_string());
                }
                temp_vec
            },
            album: album.to_string(),
            title: title.to_string(),
            track_number,
            disc_number,
        }
    }
}
