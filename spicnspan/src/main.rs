// Use:   "spicnspan",
// Short: "spicnspan removes tabs, trailing whitespace, etc. from files",
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("usage: {} <SRC> ...", args[0]);

    if args.len() < 2 {
        eprintln!("error: No filename(s) given");
        eprintln!("{}", usage);
        process::exit(1);
    }

    let mut count_formatted = 0;
    let mut total = 0;
    for filename in &args[1..] {
        match process_file(filename) {
            Err(e) => {
                eprintln!("Problem with '{}': {}", filename, e);
            }
            Ok(did_format) => {
                total += 1;
                if did_format {
                    count_formatted += 1;
                }
            }
        };
    }

    let count_unchanged = total - count_formatted;
    println!("\nAll done! ✨ 🍰 ✨");

    let mut outstats: Vec<String> = vec![];
    if count_formatted > 0 {
        outstats.push(format!("{} file(s) reformatted", count_formatted));
    }
    if count_unchanged > 0 {
        outstats.push(format!("{} file(s) left unchanged", count_unchanged));
    }
    if !outstats.is_empty() {
        println!("{}.", outstats.join(", "));
    }
}

fn process_file(filename: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().read(true).write(true).open(filename)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;

    let buf = scrub_data(&data);

    let mut is_formatted = false;
    // only overwrite if different
    if data != buf {
        // TODO: should this be moved out?
        println!("reformattted {}", filename);

        file.set_len(buf.len() as u64)?;
        file.seek(SeekFrom::Start(0))?;
        file.write_all(&buf[..])?;
        is_formatted = true;
    }

    Ok(is_formatted)
}

fn scrub_data(indata: &[u8]) -> Vec<u8> {
    // space char is ascii 0x20, this is 4 spaces
    let s: [u8; 4] = [0x20; 4];
    let mut buf: Vec<u8> = vec![];
    let mut spaces: Vec<u8> = vec![];

    for b in indata {
        match b {
            0x0d => {}                            // no-op, strip Windows-style linefeeds
            0x09 => spaces.extend_from_slice(&s), // convert all tabs to 4 space chars
            0x20 => spaces.push(*b),              // collect spaces, wait for "\n"...
            0x0a => {
                spaces.clear();
                buf.push(*b)
            } // newline, discard any saved up spaces
            _ => {
                // print any saved up spaces & reset space buffer
                buf.append(&mut spaces);
                spaces.clear();

                // now write the byte to buffer
                buf.push(*b);
            }
        }
    }

    // Ensure file ends with a newline character
    if buf.is_empty() || *buf.last().unwrap() != 0x0a {
        buf.push(0x0a);
    }

    buf
}

#[cfg(test)]
mod test {
    macro_rules! test_scrub {
        ($name:ident, $vv:expr, $r:expr) => {
            #[test]
            fn $name() {
                let input = $vv;
                let expected = $r;
                assert_eq!(super::scrub_data(&input.to_vec()), expected)
            }
        };
    }

    // TODO: more tests
    test_scrub!(test_scrub_data1, b"hello world  \n", b"hello world\n");
    test_scrub!(test_scrub_data2, b"\th", b"    h\n");
    test_scrub!(test_scrub_data3, b"\t\t\n", b"\n");
    test_scrub!(test_scrub_data4, b"\thi\r\n", b"    hi\n");
    test_scrub!(
        test_scrub_data5,
        b"\t\tfn main() -> isize {  \n\t\t\t0\n\t\t}\n",
        b"        fn main() -> isize {\n            0\n        }\n"
    );

    #[test]
    fn process_file() {
        use std::fs;
        use std::io::Write;
        use tempfile::NamedTempFile;

        let input = b"\t\tfn main() -> isize {  \n\t\t\t0\n\t\t}\n";
        let expected = b"        fn main() -> isize {\n            0\n        }\n";

        // Create a named temp file inside of `std::env::temp_dir()`.
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(input).unwrap();

        // Get the path so we can process it
        let temp_path_obj = file.into_temp_path();
        let path = temp_path_obj.to_str().unwrap();

        super::process_file(path).unwrap();

        let output = fs::read(path).unwrap();
        assert_eq!(output, expected);
    }

    test_scrub!(
        test_scrub_data6,
        b"no newline at end",
        b"no newline at end\n"
    );
    test_scrub!(test_scrub_data7, b"has newline\n", b"has newline\n");
}
