// "lc",
// "lc uppercases input sent to STDIN",

use std::io;
use std::io::BufRead;

// note: arg handling not needed b/c filter, operates on stdin
fn main() {
    let stdin = io::stdin();
    println!("{}", lc(&mut stdin.lock()));
}

fn lc(input: &mut dyn std::io::BufRead) -> String {
    let mut out: Vec<String> = vec![];
    for line in input.lines() {
        let line = line.unwrap();
        out.push(line.to_lowercase());
    }
    out.join("\n")
}

#[cfg(test)]
mod test {
    macro_rules! test_lc {
        ($name:ident, $in:expr, $exp:expr) => {
            #[test]
            fn $name() {
                let input = $in;
                let expected = $exp;
                let mut b = input.as_bytes();

                assert_eq!(super::lc(&mut b), expected);
            }
        };
    }

    test_lc!(test_lc1, "123", "123");
    test_lc!(test_lc2, "hello", "hello");
    test_lc!(test_lc3, "worLD", "world");
    test_lc!(test_lc4, "FOO", "foo");
    test_lc!(test_lc5, "OBTENÇÃO", "obtenção");
    test_lc!(test_lc6, "TSCHÜSS", "tschüss");
    test_lc!(test_lc7, "ÄËÏÖÜÂÊÔÎ", "äëïöüâêôî");
}
