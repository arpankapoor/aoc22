use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let mut split = s.split(",");
        Self {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
        }
    }
}

impl Point {
    fn distance(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }

    fn fall_down(&mut self, points_set: &HashSet<Self>) -> bool {
        for dx in [0, -1, 1] {
            let new_point = Point {
                x: self.x + dx,
                y: self.y + 1,
            };
            if !points_set.contains(&new_point) {
                *self = new_point;
                return true;
            }
        }
        false
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else {
            self.distance().total_cmp(&other.distance())
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        if end < start {
            Self {
                start: end,
                end: start,
            }
        } else {
            Self { start, end }
        }
    }

    fn points_on_line(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                points.insert(Point { x, y });
            }
        }
        points
    }
}

pub fn solve(input: String) {
    let mut point_set_input = HashSet::new();
    for line in input.lines() {
        let mut pointsplit = line.split(" -> ");
        let mut prev_point: Point = pointsplit.next().unwrap().into();
        for curr in pointsplit {
            let curr_point: Point = curr.into();
            let line = Line::new(prev_point, curr_point);
            point_set_input.extend(line.points_on_line());
            prev_point = curr_point;
        }
    }

    let start_position = Point { x: 500, y: 0 };

    let mut point_set = point_set_input.clone();
    let max_y = point_set.iter().map(|p| p.y).max().unwrap();
    let mut ans1 = 0;
    loop {
        let mut position = start_position;
        while position.y < max_y && position.fall_down(&point_set) {}

        // falling to infinity
        if position.y >= max_y {
            break;
        }

        ans1 += 1;
        point_set.insert(position);
    }

    let mut point_set = point_set_input.clone();
    let max_y = max_y + 2;
    let mut ans2 = 0;
    loop {
        let mut position = start_position;
        while (position.y < max_y - 1) && position.fall_down(&point_set) {}
        ans2 += 1;

        // haven't moved from initial position
        if position == start_position {
            break;
        }
        point_set.insert(position);
    }

    println!("ans1 = {ans1}, ans2 = {ans2}");
}
