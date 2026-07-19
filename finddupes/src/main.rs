#![allow(clippy::uninlined_format_args)]
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
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

    let mut map_hash_paths = HashMap::<Vec<u8>, Vec<PathBuf>>::new();

    for path_vec in map_size_paths.values() {
        if path_vec.len() < 2 {
            continue;
        }

        for path in path_vec {
            let hash = match sha256_file(path) {
                Ok(h) => h,
                Err(_) => continue,
            };
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

fn sha256_file(path: &Path) -> std::io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    Ok(hasher.finalize().to_vec())
}
