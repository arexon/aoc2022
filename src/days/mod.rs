mod day_01;
mod day_02;

type Run = fn(&str) -> (String, String);

pub const DAYS: [(&str, Run); 2] = [("01", day_01::run), ("02", day_02::run)];
