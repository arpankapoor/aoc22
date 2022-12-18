use std::collections::HashSet;

#[derive(Debug, Eq, Hash, Clone, Copy, PartialEq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let mut split = s.split(", ");
        let x_str = split.next().unwrap().split('=').last().unwrap();
        let y_str = split.next().unwrap().split('=').last().unwrap();
        Self {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
        }
    }
}

impl Point {
    fn manhattan_distance(&self, other: Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    // return list of points that are at given distance from us
    fn points_at_distance(&self, dist: i64, y_value: i64) -> HashSet<Self> {
        let mut points = HashSet::new();

        let y_dist = (self.y - y_value).abs();
        if y_dist > dist {
            return points;
        }

        let max_dx = dist - y_dist;
        for dx in -max_dx..=max_dx {
            let x = self.x + dx;
            points.insert(Point { x, y: y_value });
        }
        points
    }
}

struct Sensor {
    position: Point,
    closest_beacon: Point,
    radius: i64,
}

impl From<&str> for Sensor {
    fn from(s: &str) -> Self {
        let mut split = s.split(':');
        let sensor_str = split.next().unwrap();
        let beacon_str = split.next().unwrap();
        let position = sensor_str.split("at ").last().unwrap().into();
        let closest_beacon = beacon_str.split("at ").last().unwrap().into();
        Sensor {
            position,
            closest_beacon,
            radius: position.manhattan_distance(closest_beacon),
        }
    }
}

fn find_distress_signal(sensors: Vec<Sensor>, max_value: i64) -> Point {
    for y in 0..max_value {
        let mut x = 0;
        'xloop: while x < max_value {
            for sensor in &sensors {
                if sensor.radius >= sensor.position.manhattan_distance(Point { x, y }) {
                    x = 1 + sensor.position.x + (sensor.radius - (y - sensor.position.y).abs());
                    continue 'xloop;
                }
            }
            return Point { x, y };
        }
    }
    Point { x: 0, y: 0 }
}

pub fn solve(input: String) {
    const Y_VALUE: i64 = 2000000;
    let sensors: Vec<Sensor> = input.lines().map(|line| line.into()).collect();
    let sensor_and_beacon_positions: HashSet<_> = sensors
        .iter()
        .flat_map(|sensor| [sensor.position, sensor.closest_beacon])
        .collect();
    let no_beacon_positions: HashSet<_> = sensors
        .iter()
        .flat_map(|sensor| sensor.position.points_at_distance(sensor.radius, Y_VALUE))
        .filter(|p| !sensor_and_beacon_positions.contains(p))
        .collect();
    let ans1 = no_beacon_positions.len();

    const MAX_VALUE: i64 = 4000000;
    let distress_point = find_distress_signal(sensors, MAX_VALUE);
    let ans2 = distress_point.x * MAX_VALUE + distress_point.y;

    println!("ans1 = {ans1}, ans2 = {ans2}");
}
