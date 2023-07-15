use std::fs::FileType;
use clap::Parser;
use magpie_lib::Result;
use magpie_lib::{db, Args, Mode};
use walkdir::WalkDir;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.mode {
        Some(Mode::Headless) => println!("Headless Not Yet Implemented"),
        Some(Mode::Gui) => println!("GUI not yet implemented"),
        Some(Mode::Cli) => println!("CLI not yet implemented"),
        None => {}
    }

    //let pool = db::setup().await?;

    let scanner = tokio::spawn(scan());

    let result = tokio::try_join!(scanner);

    match result {
        Ok(_) => println!("Done!"),
        Err(_) => println!("Something went wrong"),
    };

    Ok(())
}

async fn scan() -> Result<()> {
    let dir = "dev/library";

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| {
        e.ok().filter(|e| {
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
    }) {
       if entry.file_type().is_dir() {
           continue
       };

        let path = entry.path();
        println!("{path:?}");
    }

    Ok(())
}
