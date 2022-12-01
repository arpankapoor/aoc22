use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const INPUT_DIR: &str = "inputs";

fn main() {
    let mut data = String::new();
    let mut f = File::open(Path::new(INPUT_DIR).join("1")).expect("input file not found");
    f.read_to_string(&mut data).expect("unable to read file");

    let mut calories: BinaryHeap<_> = data
        .split("\n\n")
        .map(|elf_food| {
            elf_food
                .split_ascii_whitespace()
                .map(|calorie| calorie.parse::<u64>().expect("invalid integer!!"))
                .sum::<u64>()
        })
        .collect();
    let max_calories = calories.pop().expect("no values?");
    println!("max calories: {}", max_calories);
    println!(
        "sum of top 3: {}",
        max_calories + calories.pop().unwrap() + calories.pop().unwrap()
    );
}
