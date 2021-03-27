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

    for filename in args.skip(1) {
        // TODO: stat filename
        // skip non-dirs

        // if filename not dir then skip
        // open dir and read all recursively
        // ignore '.' and '..'
        // hash 'file' and add PATH to list => map[hash][list of paths]
        // 'dir's should be walked
        let mut map_hash_paths = HashMap::<Vec<u8>, Vec<PathBuf>>::new();
        for entry in WalkDir::new(filename).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }

            // println!("{}", entry.path().display());
            // println!("entry: {:?}", entry);
            // println!("filename: {:?}", entry.path());

            let data = fs::read(entry.path()).unwrap();
            let hash = sha256(&data);
            // println!("{}", hex::encode(&hash));
            let list = map_hash_paths.entry(hash).or_insert(vec![]);
            list.push(entry.path().to_path_buf());
        }

        for (hash, path_vec) in &map_hash_paths {
            if path_vec.len() > 1 {
                println!("Dupe found, sha256sum: {}", hex::encode(&hash));
                for path in path_vec {
                    println!("\tpath: {}", path.display());
                }
                println!()
            }
        }
    }
}

fn sha256(buf: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(buf);

    // read hash digest and consume hasher
    hasher.finalize().to_vec()
}
