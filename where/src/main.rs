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
    // dbg!(&pattern);

    // Make pattern a regex
    let re_pattern = Regex::new(&pattern).unwrap();
    // dbg!(&re_pattern);

    let ifs = match env::var("IFS") {
        Ok(val) => val.chars().next().unwrap(),
        Err(_) => DEFAULT_IFS,
    };
    // dbg!(&ifs);

    let path = env::var("PATH").unwrap();
    for elem in path.split(ifs) {
        let path = Path::new(elem);
        if !path.exists() {
            // println!("WARN: path elem {} does not exist", elem);
            continue;
        }
        if !path.is_dir() {
            // println!("WARN: path elem {} is not a directory", elem);
            continue;
        }
        // dbg!(&elem);

        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            // dbg!(&entry);
            // Get only the basename of the entry for matching
            let basename = entry.file_name().into_string().unwrap();

            let abspath = entry.path();
            // dbg!(&abspath);

            let metadata = entry.metadata().unwrap();
            let perms = metadata.permissions().mode() & 0o0777;
            // dbg!(&perms);

            let is_file = entry.file_type().unwrap().is_file();
            let is_executable = perms & 0o111 != 0;

            // does the basename match the pattern?
            if re_pattern.is_match(&basename) && abspath.exists() && is_file && is_executable {
                println!("{}", abspath.into_os_string().into_string().unwrap());
            }
        }
    }
}
