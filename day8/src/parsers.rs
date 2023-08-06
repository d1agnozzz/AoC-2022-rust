use nom::{
    branch::alt,
    character::{
        complete::{digit1, newline},
        is_digit, is_newline,
    },
    combinator::map,
    IResult,
};

pub enum Entry {
    Digit(u8),
    NewLine,
}
pub fn parse_digit_or_newline(i: &str) -> IResult<&str, Entry> {
    alt((
        map(digit1, |c: &str| Entry::Digit(c.parse::<u8>().unwrap())),
        map(newline, |_| Entry::NewLine),
    ))(i)
}
