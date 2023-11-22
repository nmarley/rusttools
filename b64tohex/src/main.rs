use base64::{engine::general_purpose, Engine as _};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    // otherwise decode chokes on trailing newlines
    strip_trailing_newlines(&mut buffer);

    let decoded = general_purpose::STANDARD.decode(&buffer).unwrap();
    println!("{}", hex::encode(&decoded));

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
