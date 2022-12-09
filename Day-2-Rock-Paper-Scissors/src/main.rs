use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading input.txt");
    let mut result = 0;

    for line in input.trim().lines() {
        let round = Round::parse(line);
        result += round.points();
    }

    println!("Result: {}", result);
}

#[derive(Debug)]
struct Round {
    player: Shape,
    opponent: Shape,
}

impl Round {
    /// Parses a play string into the player's and opponent's shapes
    pub fn parse(play: &str) -> Self {
        let mut moves = play.split_whitespace();
        let opponent = Shape::parse(moves.next().expect("Missing opponent character"));
        let expected_result =
            MatchResult::parse(moves.next().expect("Missing expected result character"));
        let player = Shape::from_expected_result(&opponent, &expected_result);

        Self { player, opponent }
    }

    pub fn points(&self) -> u32 {
        let result = self.player.play_against(&self.opponent);
        let total_points = result.points() + self.player.points();
        total_points
    }
}

#[derive(PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn parse(input: &str) -> Self {
        match input {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!("Invalid shape {}", input),
        }
    }

    fn from_expected_result(opponent: &Self, expected_result: &MatchResult) -> Self {
        match opponent {
            Self::Rock => match expected_result {
                MatchResult::Lose => Self::Scissors,
                MatchResult::Draw => Self::Rock,
                MatchResult::Win => Self::Paper,
            },
            Self::Paper => match expected_result {
                MatchResult::Lose => Self::Rock,
                MatchResult::Draw => Self::Paper,
                MatchResult::Win => Self::Scissors,
            },
            Self::Scissors => match expected_result {
                MatchResult::Lose => Self::Paper,
                MatchResult::Draw => Self::Scissors,
                MatchResult::Win => Self::Rock,
            },
        }
    }

    fn points(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn play_against(&self, other: &Self) -> MatchResult {
        match self {
            Self::Rock => match other {
                Self::Rock => MatchResult::Draw,
                Self::Paper => MatchResult::Lose,
                Self::Scissors => MatchResult::Win,
            },
            Self::Paper => match other {
                Self::Rock => MatchResult::Win,
                Self::Paper => MatchResult::Draw,
                Self::Scissors => MatchResult::Lose,
            },
            Self::Scissors => match other {
                Self::Rock => MatchResult::Lose,
                Self::Paper => MatchResult::Win,
                Self::Scissors => MatchResult::Draw,
            },
        }
    }
}

#[derive(Debug)]
enum MatchResult {
    Win,
    Draw,
    Lose,
}

impl MatchResult {
    pub fn parse(input: &str) -> Self {
        match input {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid MatchResult input {}", input),
        }
    }

    pub fn points(&self) -> u32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}
