use std::ops::RangeBounds;

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

#[derive(Default)]
struct Rope {
    head: Position,
    tail: Position,
    tail_visited: std::collections::HashSet<Position>,
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

impl MoveVector {
    fn normalize(&self) -> MoveVector {
        use num::clamp;
        MoveVector(clamp(self.0, -1, 1), clamp(self.1, -1, 1))
    }
}
impl Rope {
    fn new() -> Rope {
        Rope::default()
    }
    fn simulate_whole_move(&mut self, head_move: MoveDirection) {
        let (distance, step) = match head_move {
            MoveDirection::Horizontal(d) => (d, MoveDirection::Horizontal(num::clamp(d, -1, 1))),
            MoveDirection::Vertical(d) => (d, MoveDirection::Vertical(num::clamp(d, -1, 1))),
        };
        // dbg!(distance, step);
        for _ in 0..distance.abs() {
            self.simulate_move_once(step);
        }
    }
    fn simulate_move_once(&mut self, head_move: MoveDirection) {
        // dbg!(head_move);
        self.move_head_once(head_move.into());
        // dbg!(self.head);
        self.move_tail_once(self.tail_catching_move());
        // dbg!(self.tail);
    }
    fn move_head_once(&mut self, move_vector: MoveVector) {
        self.head += move_vector.into();
    }
    fn move_tail_once(&mut self, move_vector: MoveVector) {
        self.tail += move_vector.into();
        self.mark_tail_visit();
    }
    fn tail_catching_move(&self) -> MoveVector {
        let delta = self.head - self.tail;
        match delta {
            Position(-1..=1, -1..=1) => MoveVector(0, 0),
            Position(-2..=2, -2..=2) => MoveVector::from(delta).normalize(),
            // Position(mut x, mut y) => {
            //     if x >= y {
            //         x -= 1;
            //     }
            //     if y >= x {
            //         y -= 1;
            //     }
            //
            //     MoveVector::from((x, y))
            // }
            _ => panic!("tail is too far away, (dx, dy) = {}, {}", delta.0, delta.1),
        }
    }
    fn mark_tail_visit(&mut self) {
        self.tail_visited.insert(self.tail);
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut rope = Rope::new();

    for line in input.lines() {
        let [dir, dist] = line.split(' ').collect::<Vec<_>>()[..] else {
            panic!("expected \"[direction] [distance]\", got: {}", line);
        };

        let dist = dist.parse::<isize>().expect("at distance parsing");

        let move_direction = match dir {
            "R" => MoveDirection::Horizontal(dist),
            "L" => MoveDirection::Horizontal(-dist),
            "U" => MoveDirection::Vertical(dist),
            "D" => MoveDirection::Vertical(-dist),
            _ => panic!("moves only U/D/L/R, got: {}", dir),
        };

        rope.simulate_whole_move(move_direction);
    }
    println!("Answer p1: {}", rope.tail_visited.len());
}
