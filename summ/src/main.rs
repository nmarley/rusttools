#![allow(clippy::uninlined_format_args)]
// summ - sum numbers from stdin and print the total
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // let mut sum: f64 = 0.0;
    // for line in io::stdin().lock().lines() {
    //     let line = line.unwrap();
    //     let num = line.parse::<f64>().unwrap();
    //     sum += num;
    // }

    let sum = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<f64>().unwrap())
        .fold(0.0, |accum, elem| accum + elem);

    println!("sum: {}", sum);
    Ok(())
}
