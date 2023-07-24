use std::fs;

#[derive(Copy, Clone)]
enum Choice {
    Rock(usize),
    Paper(usize),
    Sciscors(usize),
}

fn round(opponent: Choice, yours: Choice) -> usize {
    match opponent {
        Choice::Rock(_) => match yours {
            Choice::Rock(v) => 3 + v,
            Choice::Paper(v) => 6 + v,
            Choice::Sciscors(v) => v,
        },
        Choice::Paper(_) => match yours {
            Choice::Rock(v) => v,
            Choice::Paper(v) => 3 + v,
            Choice::Sciscors(v) => 6 + v,
        },
        Choice::Sciscors(_) => match yours {
            Choice::Rock(v) => 6 + v,
            Choice::Paper(v) => v,
            Choice::Sciscors(v) => 3 + v,
        },
    }
}

fn main() {
    let rock = Choice::Rock(1);
    let paper = Choice::Paper(2);
    let sciscors = Choice::Sciscors(3);

    let contents = fs::read_to_string("./input.txt").expect("Couldn't read from a file");
    let lines = contents.split_terminator('\n');

    let mut score_p1: usize = 0;
    let mut score_p2: usize = 0;
    for line in lines {
        let splited: Vec<&str> = line.split_terminator(' ').collect();

        let opponents_choice = match splited[0].chars().next().unwrap() {
            'A' => rock,
            'B' => paper,
            'C' => sciscors,
            _ => {
                panic!("Couldn't match opponent's letter");
            }
        };
        let your_choice_p1 = match splited[1].chars().next().unwrap() {
            'X' => rock,
            'Y' => paper,
            'Z' => sciscors,
            _ => {
                panic!("Couldn't match your letter");
            }
        };
        let your_choixe_p2 = match splited[1].chars().next().unwrap() {
            'X' => match opponents_choice {
                Choice::Rock(_) => sciscors,
                Choice::Paper(_) => rock,
                Choice::Sciscors(_) => paper,
            },
            'Y' => match opponents_choice {
                Choice::Rock(_) => rock,
                Choice::Paper(_) => paper,
                Choice::Sciscors(_) => sciscors,
            },
            'Z' => match opponents_choice {
                Choice::Rock(_) => paper,
                Choice::Paper(_) => sciscors,
                Choice::Sciscors(_) => rock,
            },
            _ => {
                panic!("Couldn't match your letter")
            }
        };

        score_p1 += round(opponents_choice, your_choice_p1);
        score_p2 += round(opponents_choice, your_choixe_p2);
    }
    println!("Part one: My score will be {score_p1}");
    println!("Part two: My score will be {score_p2}");
}
