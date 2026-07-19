use clap::Parser;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::process;

/// rando - generate a random integer between LOWER and UPPER bounds, inclusive
/// If only one argument is provided, it is the UPPER bound and the LOWER bound
/// is 0.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, override_usage = "rando [lower] upper")]
pub struct Args {
    /// The first arg
    #[clap(name = "upper")]
    pub first: u64,
    /// The second arg
    pub second: Option<u64>,
}

fn main() {
    let args = Args::parse();
    let (lower, upper) = match args.second {
        Some(second) => (args.first, second),
        None => (0, args.first),
    };
    if lower > upper {
        eprintln!(
            "error: LOWER ({}) must be less than or equal to UPPER ({})",
            lower, upper
        );
        process::exit(1);
    }

    let mut rng = SmallRng::from_entropy();
    let number = rng.gen_range(lower..=upper);
    println!("rando[{},{}]: {}", lower, upper, number);
}
