use std::io;

mod metadata;
mod scanner;
mod api;

fn main() -> Result<(), io::Error> {
    api::main();

    println!("Shutting down..");
    Ok(())
}
