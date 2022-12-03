pub fn run(input: &str) -> (String, String) {
    let input: Vec<&str> = input.split("\n\n").collect();

    (
        part_one(input.clone()).to_string(),
        part_two(input).to_string(),
    )
}

fn part_one(input: Vec<&str>) -> u32 {
    let mut input: Vec<u32> = input
        .iter()
        .map(|s| s.split('\n').map(|s| s.parse::<u32>().unwrap()).sum())
        .collect();
    input.sort_by(|a, b| b.cmp(a));
    *input.first().unwrap()
}

fn part_two(input: Vec<&str>) -> u32 {
    let mut input: Vec<u32> = input
        .iter()
        .map(|s| s.split('\n').map(|s| s.parse::<u32>().unwrap()).sum())
        .collect();
    input.sort_by(|a, b| b.cmp(a));
    input.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT.split("\n\n").collect()), 24000);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT.split("\n\n").collect()), 45000);
    }
}
