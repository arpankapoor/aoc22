use std::collections::HashSet;

pub fn solve(input: String) {
    let mut ans1 = 0;
    let mut ans2 = 0;
    for (seen, window) in input.as_bytes().windows(4).enumerate() {
        let set: HashSet<_> = window.iter().collect();
        if set.len() == 4 {
            ans1 = seen + 4;
            break;
        }
    }
    for (seen, window) in input.as_bytes().windows(14).enumerate() {
        let set: HashSet<_> = window.iter().collect();
        if set.len() == 14 {
            ans2 = seen + 14;
            break;
        }
    }
    println!("ans1: {ans1}, ans2: {ans2}");
}
