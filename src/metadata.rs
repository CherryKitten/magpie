use lofty::{Accessor, ItemKey, Tag};

#[derive(Debug)]
pub struct Track {
    artists: Vec<String>,
    album: String,
    albumartists: Vec<String>,
    title: String,
    track_number: Option<u32>,
    disc_number: Option<u32>,
}

impl Track {
    pub fn new(tag: &Tag) -> Self {
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
