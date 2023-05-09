use std::io::{self, Read};

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{digit1, hex_digit1},
    combinator::{map, map_res, opt},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
enum ASTNode {
    Number(u32),
    BinaryOp(Box<ASTNode>, Opcode, Box<ASTNode>),
}

// TODO: +, - ?
#[derive(Debug, PartialEq, Clone)]
enum Opcode {
    LeftShift,
    RightShift,
}

fn parse_number(input: &str) -> IResult<&str, ASTNode> {
    let decimal = map_res(digit1, str::parse::<u32>);
    let hex = preceded(
        tag_no_case("0x"),
        map_res(hex_digit1, |s: &str| u32::from_str_radix(s, 16)),
    );
    // order is important here
    let number = alt((hex, decimal));
    map(number, ASTNode::Number)(input)
}

// TODO: Parse op instead?
fn parse_shift(input: &str) -> IResult<&str, Opcode> {
    let left_shift = map(tag("<<"), |_| Opcode::LeftShift);
    let right_shift = map(tag(">>"), |_| Opcode::RightShift);
    alt((left_shift, right_shift))(input)
}

fn parse_expression(input: &str) -> IResult<&str, ASTNode> {
    let (input, lhs) = parse_number(input)?;
    let (input, op) = opt(parse_shift)(input)?;

    match op {
        Some(operator) => {
            let (input, rhs) = parse_number(input)?;
            let node = ASTNode::BinaryOp(Box::new(lhs), operator, Box::new(rhs));
            Ok((input, node))
        }
        None => Ok((input, lhs)),
    }
}

fn evaluate_expression(node: &ASTNode) -> u32 {
    match node {
        ASTNode::Number(value) => *value,
        ASTNode::BinaryOp(lhs, op, rhs) => {
            let left_value = evaluate_expression(lhs);
            let right_value = evaluate_expression(rhs);
            match op {
                Opcode::LeftShift => left_value << right_value,
                Opcode::RightShift => left_value >> right_value,
            }
        }
    }
}

// Put an `_` every X bytes to make it easier to read. Set this to 0 to remove
// it.
const SEP_EVERY: usize = 4;

// format the number as a binary string
fn fmt_bin_string(value: u32, sep_spacing: usize) -> String {
    let mut binary_str = format!("{:08b}", value);
    let mut formatted_str = String::new();

    for (index, ch) in binary_str.chars().enumerate() {
        if index > 0 && sep_spacing != 0 && index % sep_spacing == 0 {
            formatted_str.push('_');
        }
        formatted_str.push(ch);
    }

    formatted_str
}

fn main() -> io::Result<()> {
    // let input = "0xbabe>>8";
    // let input = "0xcafe";
    // let input = "0xface";

    // Read input from stdin
    let mut input = String::new();
    match io::stdin().read_to_string(&mut input) {
        Ok(_) => {
            // Remove newline char from input
            let input = input.trim();
            let result = parse_expression(input);
            match result {
                Ok((_, ast)) => {
                    let value = evaluate_expression(&ast);
                    let binary_str = fmt_bin_string(value, SEP_EVERY);
                    println!("{}", binary_str);
                }
                Err(error) => println!("Error: {:?}", error),
            }
        }
        Err(e) => println!("Error reading from STDIN: {:?}", e),
    };

    Ok(())
}
