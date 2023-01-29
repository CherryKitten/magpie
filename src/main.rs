use std::{fs, io};
use std::path::Path;
use lofty::{Accessor, read_from_path, TaggedFileExt};

fn main() -> Result<(), io::Error> {
    let test_path = Path::new("test_data/music");
    println!("Hello, {}!", test_path.display());
    traverse_dir(test_path);
    Ok(())
}

fn traverse_dir(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                traverse_dir(&path);
            } else {
                read_file(&path);
            }
        }
    }
    Ok(())
}

fn read_file(path: &Path) -> Result<lofty::TaggedFile, lofty::LoftyError>{
    let file = read_from_path(path)?;
    let tag = file.first_tag().unwrap();
    println!("{:?}", tag.title().unwrap());
    Ok(file)
}
