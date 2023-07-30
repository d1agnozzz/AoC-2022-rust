use itertools::{process_results, Itertools};
use std::{fs, str::FromStr};

pub(crate) mod game_outcome;
pub(crate) mod players_move;
pub(crate) mod round_p1;
pub(crate) mod round_p2;

use round_p1::RoundPart1;
use round_p2::RoundPart2;

fn main() -> color_eyre::Result<()> {
    let contents = fs::read_to_string("./input.txt").expect("Couldn't read from a file");

    let answer_p1: usize = process_results(
        contents
            .lines()
            .map(RoundPart1::from_str)
            .map_ok(RoundPart1::our_score),
        |it| it.sum(),
    )?;

    let answer_p2: usize = process_results(
        contents
            .lines()
            .map(RoundPart2::from_str)
            .map_ok(RoundPart2::our_score),
        |it| it.sum(),
    )?;

    println!("Part 1 score: {answer_p1}");
    println!("Part 2 score: {answer_p2}");
    Ok(())
}
