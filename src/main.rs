use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);


    for line in reader.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
    
}
