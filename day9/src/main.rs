use derive_more::{Add, AddAssign, Mul, Sub};

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Add, AddAssign, Sub)]
struct Position(isize, isize);
impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
impl From<(isize, isize)> for Position {
    fn from((x, y): (isize, isize)) -> Self {
        Self(x, y)
    }
}
impl From<MoveVector> for Position {
    fn from(mvec: MoveVector) -> Self {
        Self(mvec.0, mvec.1)
    }
}
impl Position {
    fn move_with(&mut self, move_vector: MoveVector) {
        *self += move_vector.into();
    }
}

#[derive(Debug, Clone, Copy)]
enum MoveDirection {
    Horizontal(isize),
    Vertical(isize),
}

#[derive(Debug, Default, PartialEq, Eq, Mul, Add, Sub, Hash, Clone, Copy)]
struct MoveVector(isize, isize);

impl From<(isize, isize)> for MoveVector {
    fn from((x, y): (isize, isize)) -> Self {
        Self(x, y)
    }
}
impl From<Position> for MoveVector {
    fn from(value: Position) -> Self {
        Self(value.0, value.1)
    }
}
impl From<MoveDirection> for MoveVector {
    fn from(move_direction: MoveDirection) -> Self {
        match move_direction {
            MoveDirection::Horizontal(dist) => Self(dist, 0),
            MoveDirection::Vertical(dist) => Self(0, dist),
        }
    }
}

use std::collections::HashSet;
#[derive(Default)]
struct Rope {
    knots: Vec<Position>,
    tail_visited: HashSet<Position>,
}

impl Rope {
    fn new(knots_num: usize) -> Rope {
        Rope {
            knots: vec![Position::default(); knots_num],
            tail_visited: HashSet::default(),
        }
    }
    fn simulate_whole_move(&mut self, head_move: MoveDirection) {
        let (distance, step) = match head_move {
            MoveDirection::Horizontal(d) => (d, MoveDirection::Horizontal(num::clamp(d, -1, 1))),
            MoveDirection::Vertical(d) => (d, MoveDirection::Vertical(num::clamp(d, -1, 1))),
        };
        for _ in 0..distance.abs() {
            let tail_position_after_move = self.simulate_move_once(step);
            self.mark_tail_visit(tail_position_after_move);
        }
    }

    /// Returns position of the tale after move simulation
    fn simulate_move_once(&mut self, head_move: MoveDirection) -> Position {
        let head = self.knots.first_mut().expect("knots are missing");
        head.move_with(head_move.into());

        let knots_len = self.knots.len();

        for i in 1..knots_len {
            if let [head, tail] = &mut self.knots[i - 1..=i] {
                tail.move_with(Rope::tail_catching_move(head, tail));
                if i == knots_len - 1 {
                    return *tail;
                }
            }
        }
        unreachable!();
    }

    /// Rope associated function, which returns `MoveVector` appropriate for catching heading
    /// knot
    fn tail_catching_move(head_position: &Position, tail_position: &Position) -> MoveVector {
        let delta = *head_position - *tail_position;
        match delta {
            Position(-1..=1, -1..=1) => MoveVector(0, 0),
            Position(x, y) => {
                use std::cmp::Ordering;

                let (mut new_x, mut new_y) = (x, y);

                if x.abs() >= y.abs() {
                    match x.cmp(&0) {
                        Ordering::Greater => new_x -= 1,
                        Ordering::Less => new_x += 1,
                        Ordering::Equal => (),
                    }
                }
                if y.abs() >= x.abs() {
                    match y.cmp(&0) {
                        Ordering::Greater => new_y -= 1,
                        Ordering::Less => new_y += 1,
                        Ordering::Equal => (),
                    }
                }

                MoveVector::from((new_x, new_y))
            }
        }
    }
    /// Add given `Position` to visited by tail position
    fn mark_tail_visit(&mut self, position: Position) {
        self.tail_visited.insert(position);
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut rope_p1 = Rope::new(2);
    let mut rope_p2 = Rope::new(10);

    for line in input.lines() {
        let [dir, dist] = line.split(' ').collect::<Vec<_>>()[..] else {
            panic!("expected \"[direction] [distance]\", got: {line}", );
        };

        let dist = dist.parse::<isize>().expect("at distance parsing");

        let move_direction = match dir {
            "R" => MoveDirection::Horizontal(dist),
            "L" => MoveDirection::Horizontal(-dist),
            "U" => MoveDirection::Vertical(dist),
            "D" => MoveDirection::Vertical(-dist),
            _ => panic!("moves only U/D/L/R, got: {dir}"),
        };

        rope_p1.simulate_whole_move(move_direction);
        rope_p2.simulate_whole_move(move_direction);
    }
    println!("Answer p1: {}", rope_p1.tail_visited.len());
    println!("Answer p2: {}", rope_p2.tail_visited.len());
}
