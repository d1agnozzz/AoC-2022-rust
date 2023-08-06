use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{char, digit1, newline},
    combinator::{map, opt},
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
pub enum Command<'a> {
    Cd { name: &'a str },
    Ls { contents: &'a str },
}

pub enum LsEntry<'a> {
    File(File),
    Directory(&'a str),
}
impl std::fmt::Debug for LsEntry<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(arg0) => write!(f, "file {} {}", arg0.name, arg0.size),
            Self::Directory(arg0) => write!(f, "dir {arg0}"),
        }
    }
}

fn take_till_command_or_eof(i: &str) -> IResult<&str, &str> {
    alt((take_until("\n$"), take_while1(|_| true)))(i)
}

fn parse_ls(i: &str) -> IResult<&str, Command> {
    let parsed_contents = preceded(tag("ls\n"), take_till_command_or_eof);
    map(parsed_contents, |s| Command::Ls { contents: s })(i)
}

fn parse_cd(i: &str) -> IResult<&str, Command> {
    let cmd = |s| Command::Cd { name: s };
    let parse_dir = preceded(tag("cd "), take_until("\n"));
    map(parse_dir, cmd)(i)
}

use super::File;

fn take_till_newline_or_eof(i: &str) -> IResult<&str, &str> {
    alt((take_until("\n"), take_while1(|_| true)))(i)
}
fn parse_file_entry(i: &str) -> IResult<&str, LsEntry> {
    let file = |s: (&str, &str)| {
        LsEntry::File(File {
            name: s.1.into(),
            size: s.0.parse::<usize>().unwrap(),
        })
    };
    let parse_file = separated_pair(digit1, char(' '), take_till_newline_or_eof);
    map(parse_file, file)(i)
}
fn parse_dir_entry<'a>(i: &'a str) -> IResult<&str, LsEntry<'a>> {
    let dir = |s: (&str, &'a str)| LsEntry::Directory(s.1);
    let parse_dir = separated_pair(
        tag("dir"),
        char(' '),
        take_till_newline_or_eof, // alt((take_until("\n"), take_while1(|_| true))),
    );
    map(parse_dir, dir)(i)
}
fn parse_dir_or_file(i: &str) -> IResult<&str, LsEntry> {
    alt((parse_dir_entry, parse_file_entry))(i)
}

fn parse_cd_or_ls(i: &str) -> IResult<&str, Command> {
    alt((parse_ls, parse_cd))(i)
}
pub fn parse_ls_entry(i: &str) -> IResult<&str, LsEntry> {
    preceded(opt(newline), parse_dir_or_file)(i)
}
pub fn parse_cmd_line(i: &str) -> IResult<&str, Command> {
    preceded(opt(newline), preceded(tag("$ "), parse_cd_or_ls))(i)
}
