use std::fs;

#[derive(Copy, Clone)]
enum Choice {
    Rock(usize),
    Paper(usize),
    Sciscors(usize),
}

fn round(opponent: Choice, yours: Choice) -> usize {
    return match opponent {
        Choice::Rock(_) => match yours {
            Choice::Rock(v) => 3 + v,
            Choice::Paper(v) => 6 + v,
            Choice::Sciscors(v) => 0 + v,
        },
        Choice::Paper(_) => match yours {
            Choice::Rock(v) => 0 + v,
            Choice::Paper(v) => 3 + v,
            Choice::Sciscors(v) => 6 + v,
        },
        Choice::Sciscors(_) => match yours {
            Choice::Rock(v) => 6 + v,
            Choice::Paper(v) => 0 + v,
            Choice::Sciscors(v) => 3 + v,
        },
    };
}

fn main() {
    let rock = Choice::Rock(1);
    let paper = Choice::Paper(2);
    let sciscors = Choice::Sciscors(3);

    let contents = fs::read_to_string("./input.txt").expect("Couldn't read from a file");
    let lines = contents.split_terminator("\n");

    let mut score_p1: usize = 0;
    let mut score_p2: usize = 0;
    for line in lines {
        let splited: Vec<&str> = line.split_terminator(" ").collect();

        let opponents_choice = match splited[0].chars().nth(0).unwrap() {
            'A' => rock.clone(),
            'B' => paper.clone(),
            'C' => sciscors.clone(),
            _ => {
                panic!("Couldn't match opponent's letter");
            }
        };
        let your_choice_p1 = match splited[1].chars().nth(0).unwrap() {
            'X' => rock.clone(),
            'Y' => paper.clone(),
            'Z' => sciscors.clone(),
            _ => {
                panic!("Couldn't match your letter");
            }
        };
        let your_choixe_p2 = match splited[1].chars().nth(0).unwrap() {
            'X' => match opponents_choice {
                Choice::Rock(_) => sciscors.clone(),
                Choice::Paper(_) => rock.clone(),
                Choice::Sciscors(_) => paper.clone(),
            },
            'Y' => match opponents_choice {
                Choice::Rock(_) => rock.clone(),
                Choice::Paper(_) => paper.clone(),
                Choice::Sciscors(_) => sciscors.clone(),
            },
            'Z' => match opponents_choice {
                Choice::Rock(_) => paper.clone(),
                Choice::Paper(_) => sciscors.clone(),
                Choice::Sciscors(_) => rock.clone(),
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
