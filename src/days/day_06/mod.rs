pub fn run(input: &str) -> (String, String) {
    (part_one(input), part_two(input))
}

fn part_one(input: &str) -> String {
    stream(input, 4).to_string()
}

fn part_two(input: &str) -> String {
    stream(input, 14).to_string()
}

fn stream(input: &str, chars: usize) -> usize {
    input
        .as_bytes()
        .windows(chars)
        .position(|c| !(1..c.len()).any(|i| c[i..].contains(&c[i - 1])))
        .unwrap()
        + chars
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), "7");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), "19");
    }
}
