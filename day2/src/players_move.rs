use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Copy, Clone, EnumIter)]
pub enum Move {
    Rock,
    Paper,
    Sciscors,
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Sciscors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {c}")),
        }
    }
}

impl Move {
    pub fn move_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Sciscors => 3,
        }
    }

    pub fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Sciscors)
                | (Self::Paper, Self::Rock)
                | (Self::Sciscors, Self::Paper)
        )
    }

    pub fn winning_move(self) -> Self {
        Self::iter()
            .find(|m| m.beats(self))
            .expect("at least one move beats us")
    }

    pub fn losing_move(self) -> Self {
        Self::iter()
            .find(|&m| self.beats(m))
            .expect("we beat at least one move")
    }

    pub fn drawing_move(self) -> Self {
        self
    }
}
