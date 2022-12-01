mod days;

pub use aoc2022::Solution;
use days::DAYS;

fn main() {
    for (day, run) in DAYS {
        let solution = Solution::new(day);
        solution.answer(run(&solution.input));
    }
}
