use std::fs::File;
use std::io::Read;
use std::path::Path;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

const INPUT_DIR: &str = "inputs";

fn main() {
    let day = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: {} dayNumber", env!("CARGO_BIN_NAME"));
        std::process::exit(1);
    });

    let mut input = String::new();
    let mut f = File::open(Path::new(INPUT_DIR).join(day.as_str())).expect("input file not found");
    f.read_to_string(&mut input).expect("unable to read file");

    match day.as_str() {
        "1" => day1::solve(input),
        "2" => day2::solve(input),
        "3" => day3::solve(input),
        "4" => day4::solve(input),
        "5" => day5::solve(input),
        "6" => day6::solve(input),
        "7" => day7::solve(input),
        "8" => day8::solve(input),
        "9" => day9::solve(input),
        "10" => day10::solve(input),
        "11" => day11::solve(input),
        "12" => day12::solve(input),
        "13" => day13::solve(input),
        _ => {
            eprintln!("invalid day: {day}");
            std::process::exit(1);
        }
    }
}
