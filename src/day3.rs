use std::collections::HashSet;

fn get_priority(c: u8) -> u8 {
    match c {
        b'a'..=b'z' => (c - b'a') + 1u8,
        b'A'..=b'Z' => (c - b'A') + 27u8,
        _ => c,
    }
}

pub fn solve(input: String) {
    let mut psum1 = 0u64;
    let mut psum2 = 0u64;
    for line in input.lines() {
        let len = line.len();
        let lt: HashSet<_> = line[..len / 2].bytes().collect();
        let rt: HashSet<_> = line[len / 2..].bytes().collect();
        let common = lt.intersection(&rt).next().copied().unwrap_or(0);
        let priority = get_priority(common);
        psum1 += priority as u64;
    }
    for line in input.lines().collect::<Vec<_>>().chunks_exact(3) {
        let l1: HashSet<_> = line[0].bytes().collect();
        let l2: HashSet<_> = line[1].bytes().collect();
        let l3: HashSet<_> = line[2].bytes().collect();
        let common = l1
            .iter()
            .find(|x| l2.contains(x) && l3.contains(x))
            .copied()
            .unwrap_or(0);
        let priority = get_priority(common);
        psum2 += priority as u64;
    }
    println!("priority1: {}, priority2: {}", psum1, psum2);
}
