#![allow(clippy::uninlined_format_args)]
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let mut args = env::args();
    if args.len() == 1 {
        eprintln!("usage: {} <dir>", args.next().unwrap());
        std::process::exit(1);
    }

    let mut map_size_paths = HashMap::<u64, Vec<PathBuf>>::new();

    for filename in args.skip(1) {
        for entry in WalkDir::new(filename).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }

            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            let size = metadata.len();
            map_size_paths
                .entry(size)
                .or_default()
                .push(entry.path().to_path_buf());
        }
    }

    let mut map_hash_paths = HashMap::<[u8; 32], Vec<PathBuf>>::new();

    for path_vec in map_size_paths.values() {
        if path_vec.len() < 2 {
            continue;
        }

        for path in path_vec {
            let hash = match blake3_file(path) {
                Ok(h) => h,
                Err(_) => continue,
            };
            map_hash_paths.entry(hash).or_default().push(path.clone());
        }
    }

    for (hash, path_vec) in &map_hash_paths {
        if path_vec.len() > 1 {
            println!("Dupe found, blake3: {}", hex::encode(hash));
            for path in path_vec {
                println!("\tpath: {}", path.display());
            }
            println!()
        }
    }
}

fn blake3_file(path: &Path) -> std::io::Result<[u8; 32]> {
    let file = File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    hasher.update_reader(file)?;
    Ok(*hasher.finalize().as_bytes())
}
