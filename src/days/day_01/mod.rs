pub fn run(input: &str) -> (String, String) {
    let input: Vec<&str> = input.split("\n\n").collect();
    let mut input: Vec<u32> = input
        .iter()
        .map(|bag| {
            bag.split('\n')
                .map(|calo| calo.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    input.sort_by(|a, b| b.cmp(a));
    let mut input = input.iter();

    (
        input.next().unwrap().to_string(),
        input.take(3).sum::<u32>().to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::Solution;

    #[test]
    fn test_part_one() {
        assert_eq!(
            run(&Solution::new("01").input),
            ("66306".to_string(), "193216".to_string())
        );
    }
}
