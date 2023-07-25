use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Couldn't read from a file");
    let lines = contents.split_terminator('\n');

    let mut counter_p1 = 0;
    let mut counter_p2 = 0;
    for line in lines {
        if let [a, b, c, d] = line
            .split(',')
            .flat_map(|e| e.split('-'))
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<_>>()[..]
        {
            let (a1, a2, b1, b2) = (a, b, c, d);
            println!("{a1}-{a2}, {b1}-{b2}");

            // part 1
            if a1 >= b1 && a2 <= b2 || a1 <= b1 && a2 >= b2{
                counter_p1 += 1;
            } 

            // part 2
            if a2 >= b1 && b2 >= a1 {
                counter_p2 += 1;
            }

        };
    }
    println!("Count part 1: {counter_p1}");
    println!("Count part 2: {counter_p2}");
}
