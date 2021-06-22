use regex::Regex;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
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

    // Make pattern a regex
    let re_pattern = Regex::new(&pattern).unwrap();
    println!("re_pattern: {:?}", re_pattern);

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
            // Get only the basename of the entry
            // println!("\tentry: {:?}", entry);
            let entry = entry.unwrap();
            let basename = entry.file_name();

            // does the basename match the pattern?
            println!("\tbasename: {:?}", basename);

            let _abspath = Path::new(&entry.path());
            let metadata = entry.metadata().unwrap();
            let mode = metadata.permissions().mode() & 0777;
            println!("mode: {:?}", mode);
            // if re_pattern.is_match(basename) && abspath.exists() && mode & WIPFINISHME != STH {}
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
