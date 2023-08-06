use strum::{EnumIter, IntoEnumIterator};

mod parsers;

struct Forest {
    tree_heights: Vec<Vec<u8>>,
}
#[derive(Debug, EnumIter)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Forest {
    fn new(rows: usize, cols: usize) -> Forest {
        Forest {
            tree_heights: vec![vec![0u8; cols]; rows],
        }
    }
    fn len(&self) -> (usize, usize) {
        (self.tree_heights.len(), self[0].len())
    }
    fn check_visibility(&self, direcion: &Direction, row: usize, col: usize) -> bool {
        let range = match direcion {
            Direction::Left => 0..col,
            Direction::Right => col + 1..self[0].len(),
            Direction::Up => 0..row,
            Direction::Down => row + 1..self.tree_heights.len(),
        };

        match direcion {
            Direction::Left | Direction::Right => {
                for tree_height in &self[row][range] {
                    if tree_height >= &self[row][col] {
                        return false;
                    }
                }
                // println!(
                //     "({},{}) = {} is visible horizontally",
                //     row, col, &self[row][col]
                // );
                true
            }
            Direction::Up | Direction::Down => {
                for i_row in &self.tree_heights[range] {
                    if i_row[col] >= self[row][col] {
                        return false;
                    }
                }
                // println!(
                //     "({},{}) = {} is visible vertically",
                //     row, col, &self[row][col]
                // );
                true
            }
        }
    }

    fn get_scenic_score(&self, row: usize, col: usize) -> usize {
        let mut total_score = 1;
        let pivoted_tree = &self[row][col];
        for direction in Direction::iter() {
            let range = match direction {
                Direction::Left => 0..col,
                Direction::Right => col + 1..self[0].len(),
                Direction::Up => 0..row,
                Direction::Down => row + 1..self.tree_heights.len(),
            };
            match direction {
                Direction::Left => {
                    let mut left_score = 0;
                    for i_col in range.rev() {
                        let tree = &self[row][i_col];
                        left_score += 1;
                        if tree >= pivoted_tree {
                            break;
                        }
                    }
                    total_score *= left_score;
                }
                Direction::Right => {
                    let mut right_score = 0;
                    let slice = &self[row][range];
                    for tree in slice {
                        right_score += 1;
                        if tree >= pivoted_tree {
                            break;
                        }
                    }
                    total_score *= right_score;
                }
                Direction::Up => {
                    let mut up_score = 0;
                    for i_row in range.rev() {
                        let tree = &self[i_row][col];
                        up_score += 1;
                        if tree >= pivoted_tree {
                            break;
                        }
                    }
                    total_score *= up_score;
                }
                Direction::Down => {
                    let mut down_score = 0;
                    let slice = &self.tree_heights[range];
                    for i_row in slice {
                        down_score += 1;
                        if i_row[col] >= *pivoted_tree {
                            break;
                        }
                    }
                    total_score *= down_score;
                }
            }
        }
        println!(
            "({},{}) = {} score is {}",
            row, col, pivoted_tree, total_score
        );
        total_score
    }
}

impl std::ops::Index<usize> for Forest {
    type Output = Vec<u8>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tree_heights[index]
    }
}

impl std::ops::IndexMut<usize> for Forest {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tree_heights[index]
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let cols = input.lines().last().unwrap().len();
    let rows = input.lines().count();
    let mut forest = Forest::new(rows, cols);

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            forest[row][col] = char.to_digit(10).unwrap() as u8;
        }
    }
    // dbg!(&forest.tree_heights);

    let mut visible_count = cols * 2 + (rows - 2) * 2;
    let mut max_scenic_score = 0;
    // dbg!(visible_count);

    for (row_i, row) in forest.tree_heights[1..forest.len().0 - 1]
        .iter()
        .enumerate()
        .map(|(i, e)| (i + 1, e))
    {
        for (col_i, col) in row[1..forest.len().1 - 1]
            .iter()
            .enumerate()
            .map(|(i, e)| (i + 1, e))
        {
            for direction in Direction::iter() {
                let visibility = forest.check_visibility(&direction, row_i, col_i);
                if visibility {
                    visible_count += 1;
                    break;
                }
            }
            let current_scenic_score = forest.get_scenic_score(row_i, col_i);
            max_scenic_score = std::cmp::max(current_scenic_score, max_scenic_score);
        }
    }
    println!("Answer p1: {}", visible_count);
    println!("Answer p2: {}", max_scenic_score)
}
