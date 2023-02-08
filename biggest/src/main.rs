#![allow(clippy::uninlined_format_args)]
use std::path::PathBuf;
use std::{env, fs, process};
use walkdir::WalkDir;

// TODO: Can add option to ignore path(s), e.g.:
// `biggest --ignore .git --ignore temp .`

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    if args.len() == 1 {
        eprintln!("usage: {} <dir>", args.next().unwrap());
        std::process::exit(1);
    }

    let mut max = 0u64;
    let mut min = u64::MAX;

    let mut biggest = PathBuf::new();
    let mut smallest = PathBuf::new();

    let mut count = 0;

    for filename in args.skip(1) {
        for entry in WalkDir::new(filename).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }
            count += 1;

            let metadata = fs::metadata(entry.path())?;
            let size = metadata.len();
            // println!("{:?}: {:?} bytes", entry.path(), size);

            if size > max {
                max = size;
                biggest = entry.path().to_path_buf();
            }
            if size < min {
                min = size;
                smallest = entry.path().to_path_buf();
            }
        }
    }

    if count > 0 {
        println!("smallest: {:?}, {} bytes", smallest, min);
        println!("biggest: {:?}, {} bytes", biggest, max);
        println!("count: {} files", count);
    } else {
        eprintln!("error: no files read");
        process::exit(1);
    }

    Ok(())
}
