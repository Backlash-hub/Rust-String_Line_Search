//Insert a string and a path to txt document. Will return lines with given string
// cargo run -- quick -- .\src\fox.txt will return "The quick"
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    
    let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("Failed to read file {}", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
