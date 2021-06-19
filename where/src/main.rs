use std::env;
use std::fs;
use std::path::Path;

const DEFAULT_IFS: char = ':';

fn main() {
    let mut args = env::args();
    if args.len() == 1 {
        eprintln!("usage: {} <pattern>", args.next().unwrap());
        std::process::exit(1);
    }
    let pattern = args.nth(1).unwrap();
    println!("pattern: {}", pattern);

    let ifs = match env::var("IFS") {
        Ok(val) => val.chars().nth(0).unwrap(),
        Err(_) => DEFAULT_IFS,
    };
    println!("ifs: {}", ifs);

    let path = env::var("PATH").unwrap();
    for elem in path.split(ifs) {
        let path = Path::new(elem);
        if !path.exists() {
            println!("WARN: path elem {} does not exist", elem);
            continue;
        }
        if !path.is_dir() {
            println!("WARN: path elem {} is not a directory", elem);
            continue;
        }
        println!("elem: {}", elem);

        for entry in fs::read_dir(path).unwrap() {
            // TODO: Make pattern a regex
            // TODO: Get only the basename of the entry
            // does the basename match the pattern?
            println!("\tentry: {:?}", entry);
        }

        // TODO:
        //  matches = matchdir(pathelem, pattern)
        //  matches.each do |m|
        //    next  unless File.exist?(m) && File.executable?(m)
        //    puts m
        //  end
    }
}

// TODO:
// def matchdir(dir, pattern)
//   entries = (Dir.entries(dir) - ['.', '..']).grep(/#{pattern}/)
//   entries.map { |entry| File.join(dir, entry) }
// end
