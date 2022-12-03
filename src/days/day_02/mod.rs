use std::{
    fmt::Debug,
    str::{FromStr, Lines},
};

pub fn run(input: &str) -> (String, String) {
    let input = input.lines();

    (
        part_one(input.clone()).to_string(),
        part_two(input).to_string(),
    )
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

fn part_one(input: Lines) -> u32 {
    let compare_score = |them, me| match (them, me) {
        (Move::Rock, Move::Paper)
        | (Move::Paper, Move::Scissors)
        | (Move::Scissors, Move::Rock) => 6,
        (them, me) if them == me => 3,
        _ => 0,
    };

    input
        .map(|moves| {
            let moves = into_vec(moves);

            moves[1] as u32 + compare_score(moves[0], moves[1])
        })
        .sum()
}

fn part_two(input: Lines) -> u32 {
    input
        .map(|moves| {
            let moves = into_vec(moves);

            match (moves[0], moves[1]) {
                (Move::Rock, Move::Rock) => 3,
                (Move::Rock, Move::Paper) => 3 + 1,
                (Move::Rock, Move::Scissors) => 6 + 2,
                (Move::Paper, Move::Rock) => 1,
                (Move::Paper, Move::Paper) => 3 + 2,
                (Move::Paper, Move::Scissors) => 6 + 3,
                (Move::Scissors, Move::Rock) => 2,
                (Move::Scissors, Move::Paper) => 3 + 3,
                (Move::Scissors, Move::Scissors) => 6 + 1,
            }
        })
        .sum()
}

fn into_vec(s: &str) -> Vec<Move> {
    s.split(' ').map(|s| s.parse::<Move>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT.lines()), 15);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT.lines()), 12);
    }
}
