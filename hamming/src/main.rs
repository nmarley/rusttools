use clap::Parser;
use std::fs;

/// hamming - calculate the Hamming distance of two files of equal size
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The first file
    pub file1: String,
    /// The second file
    pub file2: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // TODO: don't read the whole file into memory at once, use a bufreader or
    // something
    let vec1 = fs::read(&args.file1)?;
    let vec2 = fs::read(&args.file2)?;

    if vec1.len() != vec2.len() {
        return Err(format!(
            "files are not equal in length ({} != {})",
            vec1.len(),
            vec2.len()
        )
        .into());
    }

    let mut hamming_distance: u64 = 0;
    for (i, byte) in vec1.iter().enumerate() {
        if vec2[i] != *byte {
            hamming_distance += 1;
        }
    }

    println!("hamming distance: {}", hamming_distance);
    Ok(())
}
