use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::AddAssign;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn is_touching(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            "U" => Self::Up,
            _ => panic!("aaaaarrrrr!!!!"),
        }
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, dir: Direction) {
        match dir {
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
        }
    }
}

struct RopePosition<const N: usize> {
    knots: [Point; N],
}

impl<const N: usize> RopePosition<N> {
    fn new() -> Self {
        RopePosition {
            knots: [Point { x: 0, y: 0 }; N],
        }
    }

    fn move_one_step(&mut self, direction: Direction) {
        self.knots[0] += direction; // move the head
        for i in 1..N {
            // move next knot if it is not touching previous knot!!!
            if self.knots[i - 1].is_touching(&self.knots[i]) {
                break;
            }

            self.knots[i].y += match self.knots[i - 1].y.cmp(&self.knots[i].y) {
                Ordering::Greater => 1,
                Ordering::Equal => 0,
                Ordering::Less => -1,
            };

            self.knots[i].x += match self.knots[i - 1].x.cmp(&self.knots[i].x) {
                Ordering::Greater => 1,
                Ordering::Equal => 0,
                Ordering::Less => -1,
            };
        }
    }
}

fn count_tail_points_covered<const N: usize>(input: &str) -> usize {
    let mut position = RopePosition::<N>::new();
    let mut points_covered = HashSet::new();

    for line in input.lines() {
        let direction = line[0..1].into();
        let distance = line[2..].parse().unwrap();

        for _ in 0..distance {
            position.move_one_step(direction);
            points_covered.insert(position.knots[N - 1]);
        }
    }

    points_covered.len()
}

pub fn solve(input: String) {
    println!(
        "ans1 = {}, ans2 = {}",
        count_tail_points_covered::<2>(&input),
        count_tail_points_covered::<10>(&input)
    );
}
