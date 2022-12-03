pub fn run(input: &str) -> (String, String) {
    let input = input.lines().collect::<Vec<&str>>();

    (
        part_one(input.clone()).to_string(),
        part_two(input).to_string(),
    )
}

fn part_one(input: Vec<&str>) -> u32 {
    input
        .iter()
        .map(|bag| {
            let (left, right) = bag.split_at(bag.len() / 2);
            let badge = left.chars().find(|&c| right.contains(c)).unwrap();
            into_priority(badge)
        })
        .sum()
}

fn part_two(input: Vec<&str>) -> u32 {
    input
        .chunks(3)
        .map(|bag| {
            let badge = bag[0]
                .chars()
                .find(|&c| bag[1].contains(c) && bag[2].contains(c))
                .unwrap();
            into_priority(badge)
        })
        .sum()
}

fn into_priority(badge: char) -> u32 {
    match badge {
        c @ 'a'..='z' => c as u32 - 96,
        c @ 'A'..='Z' => c as u32 - 38,
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
        assert_eq!(part_one(INPUT.lines().collect()), 157);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT.lines().collect()), 70);
    }
}
