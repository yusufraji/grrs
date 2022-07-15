#![allow(unused)]
use clap::Parser;
use anyhow::{Context, Result};

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() -> Result<()> {
    let args = Cli::parse();
    println!("pattern to search for: {}", args.pattern);
    println!("file path to read: {:?}", args.path);

    let content = std::fs::read_to_string(&args.path).with_context(|| format!("could not read file '{:?}'", args.path))?;
    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}


fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}