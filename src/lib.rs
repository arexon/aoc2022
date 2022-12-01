use std::{fmt, fs};

pub struct Solution<'a> {
    pub input: String,
    day: &'a str,
}

impl<'a> Solution<'a> {
    pub fn new(day: &'a str) -> Self {
        Self {
            input: fs::read_to_string(format!("src/day_{}/input.txt", day)).unwrap(),
            day,
        }
    }

    pub fn answer<A: fmt::Display, B: fmt::Display>(&self, one: A, two: B) {
        println!(
            "aoc 2022 day {} answer:\npart one: {}\npart two: {}",
            self.day, one, two
        );
    }
}
