use std::ops::Range;

fn to_range(s: &str) -> Range<u64> {
    let mut split = s.split('-');
    let start = split.next().unwrap().parse().unwrap();
    let end = split.next().unwrap().parse().unwrap();
    Range { start, end }
}

// is r2 contained within r1?
fn is_contained_within(r1: &Range<u64>, r2: &Range<u64>) -> bool {
    r1.start <= r2.start && r2.end <= r1.end
}

// do r1 and r2 overlap at all?
fn is_overlapping(r1: &Range<u64>, r2: &Range<u64>) -> bool {
    r1.start <= r2.end && r2.start <= r1.end
}

pub fn solve(input: String) {
    let mut contained = 0u64;
    let mut overlap = 0u64;
    for line in input.lines() {
        let mut split = line.split(',');
        let r1 = to_range(split.next().unwrap());
        let r2 = to_range(split.next().unwrap());
        if is_contained_within(&r1, &r2) || is_contained_within(&r2, &r1) {
            contained += 1;
        }
        if is_overlapping(&r1, &r2) {
            overlap += 1;
        }
    }
    println!("contained = {}, overlap = {}", contained, overlap);
}
