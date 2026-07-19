// "uc",
// "uc uppercases input sent to STDIN",

use std::io;
use std::io::BufRead;

// note: arg handling not needed b/c filter, operates on stdin
fn main() {
    let stdin = io::stdin();
    println!("{}", uc(&mut stdin.lock()));
}

fn uc(input: &mut dyn std::io::BufRead) -> String {
    let mut out: Vec<String> = vec![];
    for line in input.lines() {
        let line = line.unwrap();
        out.push(line.to_uppercase());
    }
    out.join("\n")
}

#[cfg(test)]
mod test {
    macro_rules! test_uc {
        ($name:ident, $in:expr, $exp:expr) => {
            #[test]
            fn $name() {
                let input = $in;
                let expected = $exp;
                let mut b = input.as_bytes();

                assert_eq!(super::uc(&mut b), expected);
            }
        };
    }

    test_uc!(test_uc1, "123", "123");
    test_uc!(test_uc2, "hello", "HELLO");
    test_uc!(test_uc3, "worLD", "WORLD");
    test_uc!(test_uc4, "FOO", "FOO");
    test_uc!(test_uc5, "obtenção", "OBTENÇÃO");
    test_uc!(test_uc6, "tschüß", "TSCHÜSS");
    test_uc!(test_uc7, "äëïöüâêôî", "ÄËÏÖÜÂÊÔÎ");
}
