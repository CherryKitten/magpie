use crate::db::schema::{albums, artists, tracks};
use crate::metadata::Album;
use crate::{db, metadata};
use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use lofty::{
    error::{ErrorKind, LoftyError},
    read_from_path, Accessor, ItemKey, Tag, TaggedFile, TaggedFileExt,
};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn traverse_dir(dir: &Path) -> io::Result<Vec<FoundTrack>> {
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
                    Err(_) => continue,
                };
            }
        }
    }
    Ok(tracks)
}

fn read_file(path: &Path) -> Result<FoundTrack, LoftyError> {
    let file = read_from_path(path);
    return match file {
        Ok(file) => match file.first_tag() {
            Some(tag) => Ok(FoundTrack::new(tag.clone(), path)),
            None => return Err(LoftyError::new(ErrorKind::UnsupportedTag)),
        },
        Err(e) => Err(e),
    };
}

pub fn insert_found_tracks(tracks: Vec<FoundTrack>) {
    //let mut artists: Vec<Vec<String>> = tracks.iter().map(|t| t.artist.clone().unwrap()).collect();
    //let mut albumartists: Vec<Vec<String>> = tracks.iter().map(|t| t.albumartist.clone().unwrap()).collect();
    //artists.append(&mut albumartists);
    //let mut artists: Vec<&String>= artists.iter().flatten().collect();
    //artists.sort();
    //artists.dedup();

    //let mut albums: Vec<String> = tracks.iter().map(|t| t.album.clone().unwrap()).collect();

    //println!("{:?}, {:?}", artists, albums)

    let conn = &mut db::establish_connection();

    for track in tracks {
        match track.artist {
            Some(artists) => {
                for artist in artists {
                    diesel::insert_or_ignore_into(artists::table)
                        .values(artists::name.eq(artist))
                        .execute(conn).expect("TODO: panic message");
                }
            }
            None => {}
        };

        match track.albumartist {
            Some(artists) => {
                for artist in artists {
                    diesel::insert_or_ignore_into(artists::table)
                        .values(artists::name.eq(artist))
                        .execute(conn).expect("TODO: panic message");
                }
            }
            None => {}
        };

        match track.album {
            Some(ref album) => {
                diesel::insert_or_ignore_into(albums::table)
                    .values(albums::title.eq(album))
                    .execute(conn).expect("TODO: panic message");
            }
            None => {}
        };

        let found_album = albums::table
            .select(albums::id)
            .filter(albums::title.eq(&track.album))
            .first::<i32>(conn)
            .unwrap();

        diesel::insert_into(tracks::table)
            .values((
                tracks::title.eq(track.title),
                tracks::track_number.eq(track.track_number),
                tracks::disc_number.eq(track.disc_number),
                tracks::path.eq(track.path),
                tracks::year.eq(track.year),
                tracks::album.eq(found_album),
            ))
            .execute(conn)
            .expect("TODO: panic message");
    }
}

#[derive(Debug, Clone)]
pub struct FoundTrack {
    artist: Option<Vec<String>>,
    albumartist: Option<Vec<String>>,
    album: Option<String>,
    path: String,
    track_number: Option<i32>,
    disc_number: Option<i32>,
    title: Option<String>,
    year: Option<i32>,
}

impl FoundTrack {
    fn new(tag: Tag, path: &Path) -> Self {
        FoundTrack {
            artist: Some(metadata::vectorize_tags(
                tag.get_strings(&ItemKey::TrackArtist),
            )),
            albumartist: Some(metadata::vectorize_tags(
                tag.get_strings(&ItemKey::AlbumArtist),
            )),
            album: match tag.album() {
                Some(mut album) => Some(album.to_string()),
                None => None,
            },
            path: path.to_str().unwrap().to_string(),
            track_number: {
                match tag.track() {
                    Some(track) => Some(track as i32),
                    None => None,
                }
            },
            disc_number: {
                match tag.disk() {
                    Some(track) => Some(track as i32),
                    None => None,
                }
            },
            title: {
                match tag.title() {
                    Some(mut title) => Some(title.to_string()),
                    None => None,
                }
            },
            year: match tag.year() {
                Some(mut year) => Some(year as i32),
                None => None,
            },
        }
    }
}
