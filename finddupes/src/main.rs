fn main() {
    let mut args = env::args();
    if args.len() == 1 {
        eprintln!("usage: {} <dir>", args.next().unwrap());
        std::process::exit(1);
    }

    for filename in args.skip(1) {
        // if filename not dir then skip
        // open dir and read all recursively
        // ignore '.' and '..'
        // hash 'file' and add PATH to list => map[hash][list of paths]
        // 'dir's should be walked
    }
}
