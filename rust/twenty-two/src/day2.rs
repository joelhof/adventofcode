use crate::core::{Day};

pub struct DayTwo {
    input: String
}

impl From<String> for DayTwo {
    fn from(input: String) -> Self {
        DayTwo { input }
    }
}

enum Opponent {
    ROCK,
    PAPER,
    SCISSOR
}

impl From<&str> for Opponent {
    fn from(input: &str) -> Self {
        match input {
            "A" => Opponent::ROCK,
            "B" => Opponent::PAPER,
            "C" => Opponent::SCISSOR,
            _ => panic!("Unknown Rock-Paper-Scissors move {}", input)
        }
    }
}

enum YourMove {
    ROCK(u8),
    PAPER(u8),
    SCISSOR(u8)
}

impl From<&str> for YourMove {
    fn from(input: &str) -> Self {
        match input {
            "X" => YourMove::ROCK(1),
            "Y" => YourMove::PAPER(2),
            "Z" => YourMove::SCISSOR(3),
            _ => panic!("Unknown Rock-Paper-Scissors move {}", input)
        }
    }
}

struct Round(Opponent, YourMove);

impl From<&str> for Round {
    fn from(input: &str) -> Self {
        let mut i = input.split(" ");
        let o = Opponent::from(
            i.next().expect("There should be a move for your oppponent")
        );
        let y = YourMove::from(
            i.next().expect("There should be a move for you")
        );
        return Round(o, y);
    }
}

impl Round {
    fn score(&self) -> u8 {
        match (&self.0, &self.1) {
            (Opponent::ROCK, YourMove::PAPER(x)) => 6 + x,
            (Opponent::PAPER, YourMove::SCISSOR(x)) => 6 + x,
            (Opponent::SCISSOR, YourMove::ROCK(x)) => 6 + x,
            (Opponent::ROCK, YourMove::ROCK(x)) => 3 + x,
            (Opponent::PAPER, YourMove::PAPER(x)) => 3 + x,
            (Opponent::SCISSOR, YourMove::SCISSOR(x)) => 3 + x,
            (Opponent::ROCK, YourMove::SCISSOR(x)) => 0 + x,
            (Opponent::PAPER, YourMove::ROCK(x)) => 0 + x,
            (Opponent::SCISSOR, YourMove::PAPER(x)) => 0 + x,
        }
    }
 }

enum DesiredOutcome {
    WIN,
    LOSE,
    DRAW
}

impl From<&str> for DesiredOutcome {
    fn from(input: &str) -> Self {
        match input {
            "X" => DesiredOutcome::LOSE,
            "Y" => DesiredOutcome::DRAW,
            "Z" => DesiredOutcome::WIN,
            _ => panic!("Unknown Outcome {}", input)
        }
    }
}
impl DesiredOutcome {
    fn your_move(&self, opponent: &Opponent) -> YourMove {
        let your_move = match (self, opponent) {
            (DesiredOutcome::WIN, Opponent::ROCK) => "Y",
            (DesiredOutcome::LOSE, Opponent::ROCK) => "Z",
            (DesiredOutcome::DRAW, Opponent::ROCK) => "X",
            (DesiredOutcome::WIN, Opponent::PAPER) => "Z",
            (DesiredOutcome::DRAW, Opponent::PAPER) => "Y",
            (DesiredOutcome::LOSE, Opponent::PAPER) => "X",
            (DesiredOutcome::WIN, Opponent::SCISSOR) => "X",
            (DesiredOutcome::LOSE, Opponent::SCISSOR) => "Y",
            (DesiredOutcome::DRAW, Opponent::SCISSOR) => "Z",
        };
        return YourMove::from(your_move);
    }
}
impl Day for DayTwo {
    type R = u32;

    fn day() -> String where Self: Sized {
        "2".to_string()
    }

    fn part_one(&self) -> Self::R {
        self.input.lines()
            .map(|l| l.trim())
            .map(|l| Round::from(l))
            .map(|round| round.score() as u32)
            .sum()
    }

    fn part_two(&self) -> Self::R {
        self.input.lines()
            .map(|l| l.trim())
            .map(|l| {
                let s: Vec<&str> = l.split(" ").collect();
                let o = Opponent::from(s[0]);
                let d = DesiredOutcome::from(s[1]).your_move(&o);
                return Round(o, d);
            })
            .map(|round| round.score() as u32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExampleTest() {
        let input = "A Y
                           B X
                           C Z";
        let actual_res = DayTwo::from(input.to_string())
            .part_one();
        assert_eq!(15, actual_res);
    }

    #[test]
    fn partTwoExampleTest() {
        let input = "A Y
                           B X
                           C Z";
        let actual_res = DayTwo::from(input.to_string())
            .part_two();
        assert_eq!(12, actual_res);
    }
}

