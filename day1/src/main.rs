use std::fs;

#[derive(Copy, Clone)]
#[derive(Default)]
struct Elf {
    number: usize,
    calories: usize,
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Couldn't read from a file");
    let consec_nums_iterator = contents.split_terminator("\n\n");

    let mut top_three = [Elf::default(); 3];

    let mut elf_number: usize = 0;
    for lines in consec_nums_iterator {
        elf_number += 1;

        let elfs_cals = lines
            .split_terminator('\n')
            .map(|s| s.parse::<usize>().unwrap_or(0))
            .sum();
        println!("Elf {elf_number} carries {elfs_cals}");

        if elfs_cals > top_three[0].calories {
            top_three[2] = top_three[1];
            top_three[1] = top_three[0];
            top_three[0] = Elf {
                number: elf_number,
                calories: elfs_cals,
            };
        } else if elfs_cals > top_three[1].calories {
            top_three[2] = top_three[1];
            top_three[1] = Elf {
                number: elf_number,
                calories: elfs_cals,
            };
        } else if elfs_cals > top_three[2].calories {
            top_three[2] = Elf {
                number: elf_number,
                calories: elfs_cals,
            };
        }
    }

    for elf in top_three {
        println!("Elf number {0} carries {1} calories", elf.number, elf.calories);
    }
    println!("Top 3 carry {} calories", top_three.iter().map(|e| e.calories).sum::<usize>());
}
