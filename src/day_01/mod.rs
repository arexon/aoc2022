use aoc2022::Solution;

pub fn run() {
    let solution = Solution::new("01");

    let input: Vec<&str> = solution.input.split("\n\n").collect();
    let mut input: Vec<u32> = input
        .iter()
        .map(|bag| {
            bag.split('\n')
                .map(|call| call.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    input.sort_by(|a, b| b.cmp(a));
    let mut input = input.iter();

    solution.answer(input.next().unwrap(), input.take(3).sum::<u32>());
}
