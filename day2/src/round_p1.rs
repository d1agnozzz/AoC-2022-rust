use super::game_outcome::Outcome;
use super::players_move::Move;
pub struct RoundPart1 {
    opponents: Move,
    ours: Move,
}

impl RoundPart1 {
    fn outcome(self) -> Outcome {
        if self.ours.beats(self.opponents) {
            Outcome::Win
        } else if self.opponents.beats(self.ours) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    pub fn our_score(self) -> usize {
        self.ours.move_points() + self.outcome().points()
    }
}

impl std::str::FromStr for RoundPart1 {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(opponents), Some(_), Some(ours)) = (chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected \"{{theirs}} {{ours}}\", got {s:?}"));
        };
        Ok(Self {
            opponents: opponents.try_into()?,
            ours: ours.try_into()?,
        })
    }
}
