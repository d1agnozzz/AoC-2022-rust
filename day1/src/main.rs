use itertools::Itertools;
use std::{cmp, fs};

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Couldn't read from a file");

    let answer_p2 = contents
        .split("\n\n")
        .map(|e| {
            e.lines()
                .map(|l| l.parse::<usize>().expect("Can't parse"))
                .sum::<usize>()
        })
        .map(cmp::Reverse)
        .k_smallest(3)
        .enumerate()
        .map(|(i, x)| {
            if let 0 = i {
                println!("Wealthiest: {}", x.0);
            }
            x.0
        })
        .sum::<usize>();

    println!("Sum of three most wealthiest: {answer_p2}");
}
