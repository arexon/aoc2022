use std::str::Lines;

pub fn run(input: &str) -> (String, String) {
    let input = input.lines();

    (
        part_one(input.clone()).to_string(),
        part_two(input).to_string(),
    )
}

fn part_one(input: Lines) -> u32 {
    input
        .map(|assignment| {
            let (left, right) = pairs(assignment);

            left.0 <= right.0 && left.1 >= right.1 || left.1 <= right.1 && left.0 >= right.0
        })
        .fold(0, |acc, f| if f { acc + 1 } else { acc })
}

fn part_two(input: Lines) -> u32 {
    input
        .map(|assignment| {
            let (left, right) = pairs(assignment);

            left.0 <= right.1 && left.1 >= right.0
        })
        .fold(0, |acc, f| if f { acc + 1 } else { acc })
}

fn pairs(assignment: &str) -> ((u8, u8), (u8, u8)) {
    let mut assignment = assignment
        .splitn(4, &[',', '-'])
        .map(|s| s.parse::<u8>().unwrap());

    (
        (assignment.next().unwrap(), assignment.next().unwrap()),
        (assignment.next().unwrap(), assignment.next().unwrap()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT.lines()), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT.lines()), 4);
    }
}
