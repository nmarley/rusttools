#![allow(clippy::uninlined_format_args)]
// summ - sum numbers from stdin and print the total
use clap::Parser;
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Use underscores as thousand separators
    #[arg(short, long)]
    underscore: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let sum = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<f64>().unwrap())
        .fold(0.0, |accum, elem| accum + elem);

    if args.underscore {
        println!("sum: {}", format_with_underscores(sum));
    } else {
        println!("sum: {}", sum);
    }
    Ok(())
}

fn format_with_underscores(num: f64) -> String {
    let num_str = format!("{}", num as i64);

    if num_str.len() <= 3 {
        return num_str;
    }

    num_str
        .chars()
        .rev()
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if i > 0 && i % 3 == 0 {
                acc.push('_');
            }
            acc.push(c);
            acc
        })
        .chars()
        .rev()
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_with_underscores() {
        assert_eq!(format_with_underscores(123.0), "123");
        assert_eq!(format_with_underscores(1234.0), "1_234");
        assert_eq!(format_with_underscores(12345.0), "12_345");
        assert_eq!(format_with_underscores(123456.0), "123_456");
        assert_eq!(format_with_underscores(1234567.0), "1_234_567");
        assert_eq!(format_with_underscores(0.0), "0");
        assert_eq!(format_with_underscores(2022.0), "2_022");
    }
}
