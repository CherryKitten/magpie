use crate::config;
use crate::db::models::*;
use anyhow::{Error, Result};

use lofty::{read_from_path, Tag, TaggedFileExt};
use std::fs;
use std::path::Path;

pub fn do_scan() {
    println! {"Doing metadata scan"};
    let config = config::get_config();
    traverse_dir(&config.test_path).unwrap();
}

pub fn traverse_dir(dir: &Path) -> Result<()> {
    println!("Traversing though {:?}", dir);
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();

            if path.is_dir() {
                traverse_dir(&path)?;
            } else {
                match read_file(&path) {
                    Ok(_) => {}
                    Err(_) => continue,
                };
            }
        }
    }
    Ok(())
}

fn read_file(path: &Path) -> Result<()> {
    println!("Reading file {:?}", path);
    let tag = match read_from_path(path) {
        Ok(file) => read_tags(file),
        Err(error) => return Err(Error::from(error)),
    };

    if tag.is_some() {
        Track::insert_or_update(tag.unwrap(), path)?;
    }
    Ok(())
}

fn read_tags(file: lofty::TaggedFile) -> Option<Tag> {
    let tag = match file.primary_tag() {
        None => file.first_tag()?,
        Some(tag) => tag,
    };
    Some(tag.to_owned())
}
