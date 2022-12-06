mod day_00;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

type Run = fn(&str) -> (String, String);

pub const DAYS: [(&str, Run); 6] = [
    ("01", day_01::run),
    ("02", day_02::run),
    ("03", day_03::run),
    ("04", day_04::run),
    ("05", day_05::run),
    ("06", day_06::run),
];
