use super::game_outcome::Outcome;
use super::players_move::Move;

pub struct RoundPart2 {
    opponents: Move,
    outcome: Outcome,
}

impl RoundPart2 {
    fn our_move(self) -> Move {
        match self.outcome {
            Outcome::Win => self.opponents.winning_move(),
            Outcome::Draw => self.opponents.drawing_move(),
            Outcome::Loss => self.opponents.losing_move(),
        }
    }

    pub fn our_score(self) -> usize {
        self.outcome.points() + self.our_move().move_points()
    }
}

impl std::str::FromStr for RoundPart2 {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(opponents), Some(_), Some(outcome)) = (chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected \"{{theirs}} {{outcome}}\", got: {s:?}"));
        };
        Ok(RoundPart2 {
            opponents: opponents.try_into()?,
            outcome: outcome.try_into()?,
        })
    }
}
