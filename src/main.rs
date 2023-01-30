use std::io;

mod metadata;
mod scanner;
mod web;

fn main() -> Result<(), io::Error> {
    web::main();

    println!("Shutting down..");
    Ok(())
}
