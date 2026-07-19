// "l2du <timestamp>",
// "l2du parses the UNIX epoch timestamp and outputs the result as a timestamp",

use chrono::DateTime;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("error: requires a UNIX epoch timestamp");
        eprintln!("usage: {} <timestamp>", args[0]);
        process::exit(1);
    }

    let res = l2du(args[1].as_str());
    match res {
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
        Ok(dt) => {
            println!("{}", dt);
        }
    }
}

fn l2du(ts: &str) -> Result<String, Box<dyn std::error::Error>> {
    let secs = ts.parse::<i64>()?;

    let dt = DateTime::from_timestamp(secs, 0)
        .ok_or("Invalid timestamp")?
        .naive_utc();

    let res = dt.format("%Y-%m-%d %H:%M:%S UTC").to_string();

    Ok(res)
}

#[cfg(test)]
mod test {
    //use super::*;
    #[test]
    fn l2du() {
        let input = "1136214245";
        let expected = "2006-01-02 15:04:05 UTC";
        assert_eq!(super::l2du(input).unwrap(), expected);

        let input = "bad input";
        assert!(super::l2du(input).is_err());
    }
}
