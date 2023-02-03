use std::io;

mod metadata;
mod scanner;
mod api;
mod db;

fn main() -> Result<(), io::Error> {
    api::main();

    println!("Shutting down..");
    Ok(())
}
