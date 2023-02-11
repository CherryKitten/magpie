use crate::config;
use crate::db::models::*;
use anyhow::{Error, Result};

use lofty::{read_from_path, Tag, TaggedFileExt};
use log::{error, info, trace};
use std::fs;
use std::path::Path;

pub fn do_scan() {
    info!("Doing metadata scan");
    let config = config::get_config();
    match traverse_dir(&config.test_path) {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e)
        }
    }
}

pub fn traverse_dir(dir: &Path) -> Result<()> {
    info!("Traversing through {:?}", dir);
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
    trace!("Reading file {:?}", path);
    let tag = match read_from_path(path) {
        Ok(file) => read_tags(file),
        Err(e) => return Err(Error::from(e)),
    };

    if tag.is_some() {}

    match tag {
        None => {
            error!("No tag found in {:?}", path);
        }
        Some(tag) => {
            Track::insert_or_update(tag, path)?;
        }
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
