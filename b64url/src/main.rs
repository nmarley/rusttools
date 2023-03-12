use base64::{engine::general_purpose, Engine as _};
use clap::Parser;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Disable hex output
    #[clap(long = "disable-hex", default_value_t=false)]
    pub disable_hex: bool,

    /// Disable str output
    #[clap(long = "disable-str", default_value_t=false)]
    pub disable_str: bool,

    /// Suppress field labels
    #[clap(long = "suppress-labels", default_value_t=false)]
    pub suppress_labels: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    // otherwise decode chokes on trailing newlines
    strip_trailing_newlines(&mut buffer);

    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(&buffer).unwrap();

    if !args.disable_hex {
        let field_name = match args.suppress_labels {
            true => "",
            false => "hex: ",
        };
        println!("{}{}", field_name, hex::encode(&decoded));
    }

    if !args.disable_str {
        match std::str::from_utf8(&decoded) {
            Ok(u) => {
                let field_name = match args.suppress_labels {
                    true => "",
                    false => "str: ",
                };
                println!("{}{}", field_name, u);
            }
            Err(_e) => {
                println!("invalid utf8 - unable to convert")
            }
        };
    }

    Ok(())
}

// TODO: prep for testing, but don't have time
// fn b64decode(encoded: &[u8]) -> Result<Vec<u8>, base64::DecodeError> {
//     let decoded = general_purpose::URL_SAFE_NO_PAD.decode(&buffer).unwrap();
// }

fn strip_trailing_newlines(buffer: &mut Vec<u8>) {
    // remove trailing newline, if exists
    while buffer[buffer.len() - 1] == b'\n' || buffer[buffer.len() - 1] == b'\r' {
        _ = buffer.pop();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn trailing_newlines() {
        let mut u8vec: Vec<u8> = b"hello world\n\r\n\r".to_vec();
        super::strip_trailing_newlines(&mut u8vec);
        assert_eq!(b"hello world".to_vec(), u8vec);
    }
}
