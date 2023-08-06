use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldn't read from a file");
    let mut set_p1 = std::collections::HashSet::new();
    let mut set_p2 = std::collections::HashSet::new();

    for i in 0..input.len() {
        let mut marker_found = true;
        for j in i..i + 4 {
            let char = input.chars().nth(j).unwrap();
            if set_p1.contains(&char) {
                marker_found = false;
                set_p1.clear();
                break;
            } else {
                set_p1.insert(char);
            }
        }
        if marker_found {
            println!("Start-of-packer found at {}", i + 4);
            break;
        }
    }
    for i in 0..input.len() {
        let mut marker_found = true;
        for j in i..i + 14 {
            let char = input.chars().nth(j);
            if set_p2.contains(&char) {
                marker_found = false;
                set_p2.clear();
                break;
            } else {
                set_p2.insert(char);
            }
        }
        if marker_found {
            println!("Start-of-message found at {}", i + 14);
        }
    }

}
