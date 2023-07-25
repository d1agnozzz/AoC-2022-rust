use std::fs;

fn main() {
    let contents = fs::read_to_string("./large.txt").expect("Couldn't read from file");
    let stacks_count = (contents.lines().next().unwrap().len() + 1) / 4;
    let mut vectors_p1 = vec![Vec::<char>::new(); stacks_count];
    let mut vectors_p2 = vec![Vec::<char>::new(); stacks_count];
    for line in contents.lines() {
        let bracket_indices: Vec<_> = line.match_indices('[').map(|(i, _)| i).collect();

        if !bracket_indices.is_empty() {
            for i in bracket_indices {
                vectors_p1[i / 4].insert(
                    0,
                    line.chars()
                        .nth(i + 1)
                        .expect("Couldn't find nth character"),
                );
                vectors_p2[i / 4].insert(
                    0,
                    line.chars()
                        .nth(i + 1)
                        .expect("Couldn't find nth character"),
                );
            }
        } else if line.contains("move") {
            let instruction: Vec<_> = line.split_terminator(' ').filter_map(|e| e.parse::<usize>().ok()).collect();
            if let [quantity, src, dest] = instruction[..] {
                for _ in 0..quantity {
                    if let Some(e) = vectors_p1[src-1].pop(){
                        vectors_p1[dest-1].push(e);
                    }
                }
                let range = vectors_p2[src-1].len() - quantity ..;
                let taken: Vec<_> = vectors_p2[src-1].drain(range).collect();
                vectors_p2[dest-1].extend_from_slice(&taken);

            }
        }
    }

    // for vec in &vectors {
    //     println!("{vec:?}");
    // }
    let mut answer_p1 = String::new();
    for vec in &vectors_p1 {
        if let Some(c) = vec.last() {
            answer_p1.push(*c);
        }
    }
    let mut answer_p2 = String::new();
    for vec in &vectors_p2 {
        if let Some(c) = vec.last() {
            answer_p2.push(*c);
        }
    }
    println!("The answer for part 1 is: {answer_p1}");
    println!("The answer for part 2 is: {answer_p2}");
}
