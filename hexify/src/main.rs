use std::io;

// tool does this:
//
// echo 08 07 65 b5 02 06 04 71 f6 | hexify
// 0x08, 0x07, 0x65, 0xb5, 0x02, 0x06, 0x04, 0x71, 0xf6
//
// useful when editing in vim and easy to call w/a single command

fn main() -> io::Result<()> {
    let buffer = io::read_to_string(io::stdin())?;

    let rv = buffer
        .trim()
        .split_whitespace()
        .map(|word| format!("0x{}", word))
        .collect::<Vec<_>>()
        .join(", ");
    println!("{}", rv);

    Ok(())
}
