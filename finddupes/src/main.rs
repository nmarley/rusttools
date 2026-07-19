#![allow(clippy::uninlined_format_args)]
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
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

    let mut map_hash_paths = HashMap::<Vec<u8>, Vec<PathBuf>>::new();

    for path_vec in map_size_paths.values() {
        if path_vec.len() < 2 {
            continue;
        }

        for path in path_vec {
            let data = match fs::read(path) {
                Ok(d) => d,
                Err(_) => continue,
            };
            let hash = sha256(&data);
            map_hash_paths.entry(hash).or_default().push(path.clone());
        }
    }

    for (hash, path_vec) in &map_hash_paths {
        if path_vec.len() > 1 {
            println!("Dupe found, sha256sum: {}", hex::encode(hash));
            for path in path_vec {
                println!("\tpath: {}", path.display());
            }
            println!()
        }
    }
}

fn sha256(buf: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(buf);
    hasher.finalize().to_vec()
}
