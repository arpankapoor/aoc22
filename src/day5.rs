use std::collections::VecDeque;

pub fn solve(input: String) {
    let mut split = input.split("\n\n");
    let stack_str: Vec<_> = split.next().unwrap().lines().collect();
    let rproc = split.next().unwrap();

    let nstacks = (stack_str[0].len() + 1) / 4;
    let mut stacks = vec![VecDeque::new(); nstacks];
    for stackline in &stack_str[..stack_str.len() - 1] {
        for sno in 0..nstacks {
            let start = 4 * sno;
            let end = (4 * (sno + 1)) - 1;
            let element = stackline[start..end].trim();
            if element.is_empty() {
                continue;
            }

            stacks[sno].push_back(element.as_bytes()[1] as char);
        }
    }

    let mut stacks2 = stacks.clone();
    for line in rproc.lines() {
        let split: Vec<_> = line.split(' ').collect();
        let n: usize = split[1].parse().unwrap();
        let src: usize = split[3].parse().unwrap();
        let dst: usize = split[5].parse().unwrap();

        for _ in 0..n {
            let element = stacks[src - 1].pop_front().unwrap();
            stacks[dst - 1].push_front(element);
        }

        let mut tmp = VecDeque::new();
        for _ in 0..n {
            let element = stacks2[src - 1].pop_front().unwrap();
            tmp.push_front(element);
        }
        for e in tmp {
            stacks2[dst - 1].push_front(e);
        }
    }
    let mut ans1 = String::new();
    for stack in &mut stacks {
        ans1.push(stack.pop_front().unwrap());
    }

    let mut ans2 = String::new();
    for stack in &mut stacks2 {
        ans2.push(stack.pop_front().unwrap());
    }
    println!("ans1 = {ans1}, ans2 = {ans2}");
}
