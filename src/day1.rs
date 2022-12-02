use std::collections::BinaryHeap;

pub fn solve(input: String) {
    let mut calories: BinaryHeap<_> = input
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
