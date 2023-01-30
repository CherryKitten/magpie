use std::io;
use std::path::Path;

mod scanner;
mod metadata;
mod web;
use crate::scanner::traverse_dir;

fn main() -> Result<(), io::Error> {


    web::main();

    println!("Shutting down..");
    Ok(())
}

