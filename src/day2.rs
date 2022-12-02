use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => panic!("invalid game strategy"),
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Rock, Self::Scissor)
            | (Self::Scissor, Self::Paper)
            | (Self::Paper, Self::Rock) => Ordering::Greater,
            (x, y) if (*x as u8) == (*y as u8) => Ordering::Equal,
            _ => Ordering::Less,
        }
    }
}

struct Game {
    opponent: Shape,
    own: Shape,
}

impl Game {
    fn result(&self) -> MatchResult {
        match self.own.cmp(&self.opponent) {
            Ordering::Less => MatchResult::Loss,
            Ordering::Equal => MatchResult::Draw,
            Ordering::Greater => MatchResult::Win,
        }
    }
}

enum MatchResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl From<&str> for MatchResult {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("invalid game result"),
        }
    }
}

impl MatchResult {
    fn get_own_shape(&self, opponent: Shape) -> Shape {
        match self {
            Self::Loss => match opponent {
                Shape::Paper => Shape::Rock,
                Shape::Rock => Shape::Scissor,
                Shape::Scissor => Shape::Paper,
            },
            Self::Draw => opponent,
            Self::Win => match opponent {
                Shape::Paper => Shape::Scissor,
                Shape::Rock => Shape::Paper,
                Shape::Scissor => Shape::Rock,
            },
        }
    }
}

pub fn solve(input: String) {
    let mut points1 = 0u64;
    let mut points2 = 0u64;
    for game in input.lines() {
        let mut game = game.split(' ');
        let opponent: Shape = game.next().expect("opponent value missing!").into();
        let second_str = game.next().expect("self value!");
        let own1 = second_str.into();
        let result: MatchResult = second_str.into();
        let game = Game {
            opponent,
            own: own1,
        };
        points1 += (own1 as u64) + (game.result() as u64);
        points2 += (result.get_own_shape(opponent) as u64) + (result as u64);
    }
    println!("total points1: {}, total points2: {}", points1, points2);
}
