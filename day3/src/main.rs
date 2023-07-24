use std::{collections, fs};

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Couldn't read from file");
    let lines = contents.split_terminator('\n');

    let mut sum_of_commons_p1 = 0;
    let mut sum_of_badges_p2 = 0;

    let mut sets_p2 = vec![collections::HashSet::new(); 3];

    let mut group = 0_usize;

    for line in lines {
        let mut part_1 = collections::HashSet::new();
        let length = line.len();
        let first_half = &line[0..(length / 2)];
        let second_half = &line[length / 2..length];
        println!("{line}");
        println!("1: {first_half}");
        println!("2: {second_half}");

        for char in first_half.chars() {
            part_1.insert(char);
        }
        for char in second_half.chars() {
            if part_1.contains(&char) {
                println!("Common is {char}");
                if char.is_lowercase() {
                    println!("Priority is {}", char as usize - 96);
                    sum_of_commons_p1 += char as usize - 96;
                } else if char.is_uppercase() {
                    println!("Priority is {}", char as usize - 64 + 26);
                    sum_of_commons_p1 += char as usize - 64 + 26;
                }
                break;
            }
        }


        if group != 2 {
            for char in line.chars() {
                sets_p2[group].insert(char);
            }
            group += 1;
        } else {
            for char in line.chars() {
                sets_p2[group].insert(char);
            }
            println!("set1: {:?}\\nset2: {:?}\\nset3: {:?}", sets_p2[0], sets_p2[1], sets_p2[2]);
            let intersec: collections::HashSet<char> =
                sets_p2[0].intersection(&sets_p2[1]).cloned().collect();
            println!("intersec1: {:?}", intersec);
            let intersec: Vec<char> = intersec.intersection(&sets_p2[2]).cloned().collect();
            println!("intersec2: {:?}", intersec);
            let badge = intersec[0];

            let mut priority = 0;
            if badge.is_lowercase() {
                priority = badge as usize - 96;
            } else if badge.is_uppercase() {
                priority = badge as usize - 64 + 26;
            }
            println!("Common badge is {badge} {priority}");
            sum_of_badges_p2 += priority;

            group = 0;
            for set in &mut sets_p2 {
                set.clear();
            }
        }
    }
    println!("The sum of commons is {sum_of_commons_p1}");
    println!("The sum of badge priorities is {sum_of_badges_p2}");
}
