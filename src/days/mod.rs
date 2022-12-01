mod day_01;

type Run = fn(&str) -> (String, String);

pub const DAYS: [(&str, Run); 1] = [("01", day_01::run)];
