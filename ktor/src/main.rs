#![allow(clippy::uninlined_format_args)]
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let s = System::new_all();
    for (pid, process) in s.processes() {
        println!("{} {}", pid, process.name());
    }
}
