use std::{fs, io};
use std::path::Path;

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
                println!("{:?}", path);
            }
        }
    }
    Ok(())
}
