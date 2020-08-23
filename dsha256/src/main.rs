// double sha256
use sha2::{Sha256, Digest};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    let mut hasher = Sha256::new();

    // write input message
    hasher.update(buffer);

    // read hash digest and consume hasher
    let hash1 = hasher.finalize();
    // println!("digest1: {:x}", hash1);

    hasher = Sha256::new();
    hasher.update(hash1);
    let hash2 = hasher.finalize();

    // println!("digest2: {:x}", hash2);

    println!("{:x}", hash2);
    Ok(())
}
