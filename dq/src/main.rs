use clap::Parser;
use std::fs;
use std::io::{Read, Seek, Write};

/// dq (dumb quotes) - replace smart quotes with dumb quotes
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = Some("dq (dumb quotes) replaces smart quotes with dumb quotes, used primarily when an LLM generates a block of text with smart quotes that you didn't want"))]
pub struct Args {
    /// Also replace em dashes (—) with double hyphens (--)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    em_dash: bool,

    /// Also replace en dashes (–) with single hyphens (-)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    en_dash: bool,

    /// Files to process
    #[arg(required = true)]
    files: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut count_formatted = 0;
    let mut total = 0;

    for filename in &args.files {
        match process_file(filename, args.em_dash, args.en_dash) {
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

    Ok(())
}

fn process_file(
    filename: &str,
    em_dash: bool,
    en_dash: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let buf = process_text(&data, em_dash, en_dash);

    let mut is_formatted = false;
    if data != buf {
        file.set_len(buf.len() as u64)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        file.write_all(buf.as_bytes())?;
        is_formatted = true;
    }

    Ok(is_formatted)
}

fn process_text(data: &str, em_dash: bool, en_dash: bool) -> String {
    let mut buf = data
        .replace("“", "\"")
        .replace("”", "\"")
        .replace("‘", "'")
        .replace("’", "'");

    if em_dash {
        buf = buf.replace("—", "--");
    }
    if en_dash {
        buf = buf.replace("–", "-");
    }

    buf
}

#[cfg(test)]
mod test {
    macro_rules! test_process {
        ($name:ident, $input:expr, $expected:expr, $em:expr, $en:expr) => {
            #[test]
            fn $name() {
                assert_eq!(super::process_text($input, $em, $en), $expected)
            }
        };
    }

    test_process!(
        test_double_open,
        "“Hello” world",
        "\"Hello\" world",
        false,
        false
    );
    test_process!(
        test_double_close,
        "Hello “world”",
        "Hello \"world\"",
        false,
        false
    );
    test_process!(
        test_single_open,
        "‘Hello’ world",
        "'Hello' world",
        false,
        false
    );
    test_process!(
        test_single_close,
        "Hello ‘world’",
        "Hello 'world'",
        false,
        false
    );
    test_process!(
        test_mixed,
        "‘Don’t’ use “smart” quotes.",
        "'Don\'t' use \"smart\" quotes.",
        false,
        false
    );
    test_process!(
        test_no_smart_quotes,
        "Hello world with 'dumb' quotes.",
        "Hello world with 'dumb' quotes.",
        false,
        false
    );
    test_process!(test_empty, "", "", false, false);

    // Tests for em dash
    test_process!(
        test_em_dash_off,
        "An em dash—right here.",
        "An em dash—right here.",
        false,
        false
    );
    test_process!(
        test_em_dash_on,
        "An em dash—right here.",
        "An em dash--right here.",
        true,
        false
    );
    test_process!(
        test_em_dash_on_with_quotes,
        "‘Don’t’ use “smart” quotes—or em dashes.",
        "'Don\'t' use \"smart\" quotes--or em dashes.",
        true,
        false
    );

    // Tests for en dash
    test_process!(
        test_en_dash_off,
        "An en dash–right here.",
        "An en dash–right here.",
        false,
        false
    );
    test_process!(
        test_en_dash_on,
        "An en dash–right here.",
        "An en dash-right here.",
        false,
        true
    );
    test_process!(
        test_en_dash_on_with_quotes,
        "‘Don’t’ use “smart” quotes–or en dashes.",
        "'Don\'t' use \"smart\" quotes-or en dashes.",
        false,
        true
    );
    test_process!(
        test_both_dashes_on_with_quotes,
        "‘Don’t’ use “smart” quotes—or em dashes–or en dashes.",
        "'Don\'t' use \"smart\" quotes--or em dashes-or en dashes.",
        true,
        true
    );

    #[test]
    fn process_file_integration_no_em_dash() {
        use std::fs;
        use std::io::Write;
        use tempfile::NamedTempFile;

        let input = "‘Don’t’ use “smart” quotes—or em dashes.";
        let expected = "'Don\'t' use \"smart\" quotes—or em dashes."; // Em dash unchanged

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(input.as_bytes()).unwrap();

        let temp_path_obj = file.into_temp_path();
        let path = temp_path_obj.to_str().unwrap();

        super::process_file(path, false, false).unwrap(); // em_dash is false

        let output = fs::read_to_string(path).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn process_file_integration_with_em_dash() {
        use std::fs;
        use std::io::Write;
        use tempfile::NamedTempFile;

        let input = "‘Don’t’ use “smart” quotes—or em dashes.";
        let expected = "'Don\'t' use \"smart\" quotes--or em dashes."; // Em dash replaced

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(input.as_bytes()).unwrap();

        let temp_path_obj = file.into_temp_path();
        let path = temp_path_obj.to_str().unwrap();

        super::process_file(path, true, false).unwrap(); // em_dash is true

        let output = fs::read_to_string(path).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn process_file_integration_with_en_dash() {
        use std::fs;
        use std::io::Write;
        use tempfile::NamedTempFile;

        let input = "Text with an en dash– like so.";
        let expected = "Text with an en dash- like so."; // En dash replaced

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(input.as_bytes()).unwrap();

        let temp_path_obj = file.into_temp_path();
        let path = temp_path_obj.to_str().unwrap();

        super::process_file(path, false, true).unwrap(); // en_dash is true

        let output = fs::read_to_string(path).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn process_file_integration_with_both_dashes() {
        use std::fs;
        use std::io::Write;
        use tempfile::NamedTempFile;

        let input = "Text with an en dash– and an em dash— like so.";
        let expected = "Text with an en dash- and an em dash-- like so."; // Both replaced

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(input.as_bytes()).unwrap();

        let temp_path_obj = file.into_temp_path();
        let path = temp_path_obj.to_str().unwrap();

        super::process_file(path, true, true).unwrap(); // Both flags true

        let output = fs::read_to_string(path).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn process_file_integration_no_dashes() {
        use std::fs;
        use std::io::Write;
        use tempfile::NamedTempFile;

        let input = "Text with an en dash– and an em dash— like so.";
        let expected = "Text with an en dash– and an em dash— like so."; // Both unchanged

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(input.as_bytes()).unwrap();

        let temp_path_obj = file.into_temp_path();
        let path = temp_path_obj.to_str().unwrap();

        super::process_file(path, false, false).unwrap(); // Both flags false

        let output = fs::read_to_string(path).unwrap();
        assert_eq!(output, expected);
    }
}
