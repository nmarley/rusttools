#![allow(clippy::uninlined_format_args)]
// double sha256
use sha2::{Digest, Sha256};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    let vec_hash = dsha256(&buffer);

    println!("{}", hex::encode(vec_hash));
    Ok(())
}

/// Performs a double sha256 hash.
///
/// ```
/// assert_eq!(dsha256(), "")
/// ```
fn dsha256(buf: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(buf);

    // read hash digest and consume hasher
    let hash1 = hasher.finalize();
    // println!("digest1: {:x}", hash1);

    hasher = Sha256::new();
    hasher.update(hash1);

    hasher.finalize().to_vec()
}
