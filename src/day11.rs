enum Operand {
    OldValue,
    Literal(usize),
}

impl From<&str> for Operand {
    fn from(s: &str) -> Self {
        match s {
            "old" => Self::OldValue,
            x => Self::Literal(x.parse().unwrap()),
        }
    }
}

enum Operator {
    Add,
    Mul,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => panic!("aaaarrrrr!!!"),
        }
    }
}

struct Operation {
    lhs: Operand,
    rhs: Operand,
    op: Operator,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        let mut operation = s.split(" = ").skip(1).next().unwrap().split(" ");
        let lhs = operation.next().unwrap().into();
        let op = operation.next().unwrap().into();
        let rhs = operation.next().unwrap().into();
        Self { lhs, rhs, op }
    }
}

impl Operation {
    fn perform_operation(&self, old: usize) -> usize {
        let lhs = match self.lhs {
            Operand::OldValue => old,
            Operand::Literal(x) => x,
        };
        let rhs = match self.rhs {
            Operand::OldValue => old,
            Operand::Literal(x) => x,
        };
        match self.op {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }
}

struct DivTest {
    div_by: usize,
    true_monkey: usize,
    false_monkey: usize,
}

impl DivTest {
    fn from_lines<'a>(mut lines: impl Iterator<Item = &'a str>) -> Self {
        let mut get_last_num = || {
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap()
        };
        let div_by = get_last_num();
        let true_monkey = get_last_num();
        let false_monkey = get_last_num();
        Self {
            div_by,
            true_monkey,
            false_monkey,
        }
    }
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    div_test: DivTest,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        // skip line 1: Monkey <no>
        let mut lines = s.lines().skip(1);
        let items = lines
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(", ")
            .map(|worry| worry.parse().unwrap())
            .collect();
        let operation = lines
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .into();
        let div_test = DivTest::from_lines(lines);
        Self {
            items,
            operation,
            div_test,
        }
    }
}

impl Monkey {
    fn inspect_items(&mut self, worry_div_by: usize, modulus: usize) -> Vec<usize> {
        self.items
            .drain(..)
            .map(|item| (self.operation.perform_operation(item) / worry_div_by) % modulus)
            .collect()
    }
}

fn calc_monkey_business(input: &str, rounds: usize, worry_div_by: usize) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|x| x.into()).collect();
    let modulus: usize = monkeys.iter().map(|m| m.div_test.div_by).product();
    let nmonkeys = monkeys.len();
    let mut inspected_items = vec![0; nmonkeys];
    for _round in 0..rounds {
        for mid in 0..nmonkeys {
            let new_items = monkeys[mid].inspect_items(worry_div_by, modulus);
            inspected_items[mid] += new_items.len();
            for item in new_items {
                let dest_mid = if item % monkeys[mid].div_test.div_by == 0 {
                    monkeys[mid].div_test.true_monkey
                } else {
                    monkeys[mid].div_test.false_monkey
                };
                monkeys[dest_mid].items.push(item);
            }
        }
    }
    inspected_items.sort();
    inspected_items[nmonkeys - 1] * inspected_items[nmonkeys - 2]
}

pub fn solve(input: String) {
    println!(
        "ans1={}, ans2={}",
        calc_monkey_business(&input, 20, 3),
        calc_monkey_business(&input, 10000, 1)
    );
}
