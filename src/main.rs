use std::io;
use std::path::Path;

mod scanner;
mod metadata;
use crate::scanner::traverse_dir;

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

