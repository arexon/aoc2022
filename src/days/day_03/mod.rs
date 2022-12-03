use std::{ops::ControlFlow, str::Lines};

pub fn run(input: &str) -> (String, String) {
    let input = input.lines();

    (
        part_one(input.clone()).to_string(),
        part_two(input).to_string(),
    )
}

fn part_one(input: Lines) -> u32 {
    input
        .map(|s| {
            let (left, right) = s.split_at(s.len() / 2);
            let shared_char = left.chars().try_for_each(|c| {
                if right.contains(c) {
                    ControlFlow::Break(c)
                } else {
                    ControlFlow::Continue(())
                }
            });
            match shared_char {
                ControlFlow::Break(c) => c,
                _ => unreachable!(),
            }
        })
        .map(into_priority)
        .sum()
}

fn part_two(input: Lines) -> u32 {
    input
        .clone()
        .step_by(3)
        .zip(
            input
                .clone()
                .skip(1)
                .step_by(3)
                .zip(input.skip(2).step_by(3)),
        )
        .map(|(s1, (s2, s3))| {
            let badge = s1.chars().try_for_each(|c| {
                if s2.contains(c) && s3.contains(c) {
                    ControlFlow::Break(c)
                } else {
                    ControlFlow::Continue(())
                }
            });
            match badge {
                ControlFlow::Break(c) => c,
                _ => unreachable!(),
            }
        })
        .map(into_priority)
        .sum()
}

fn into_priority(c: char) -> u32 {
    match c {
        c @ 'A'..='Z' => c as u32 - 38,
        c @ 'a'..='z' => c as u32 - 96,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT.lines()), 157);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT.lines()), 70);
    }
}
