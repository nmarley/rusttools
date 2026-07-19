// "longest",
// "Longest prints the longest of a list of lines in STDIN",

use std::io;
use std::io::BufRead;

// note: arg handling not needed b/c filter, operates on stdin
fn main() {
    let stdin = io::stdin();
    println!("{}", longest(&mut stdin.lock()));
}

fn longest(input: &mut dyn std::io::BufRead) -> String {
    let mut longest: String = String::from("");
    for line in input.lines() {
        let line = line.unwrap();
        if line.len() > longest.len() {
            longest = line;
        }
    }
    longest
}

#[cfg(test)]
mod test {
    // use super::*;
    macro_rules! test_longest {
        ($name:ident, $in:expr, $exp:expr) => {
            #[test]
            fn $name() {
                let input = $in;
                let expected = $exp;
                let mut b = input.as_bytes();

                assert_eq!(super::longest(&mut b), expected);
            }
        };
    }

    test_longest!(test_longest1, "123\n12345\n1234\n", "12345");
    test_longest!(test_longest2, "123 12345 1234\n", "123 12345 1234");
    test_longest!(test_longest3, "123 12345 1234", "123 12345 1234");
    test_longest!(
        test_longest4,
        "albuquerque\npassamaquaddy\nlondon",
        "passamaquaddy"
    );
    test_longest!(
        test_longest5,
        "albuquerque\npassamaquaddy\nlondonderryinthemorning",
        "londonderryinthemorning"
    );
}
