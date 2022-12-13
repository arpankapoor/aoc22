use std::cmp::Ordering;

#[derive(Clone, Debug, Eq)]
enum PacketData {
    Integer(usize),
    List(Vec<PacketData>),
}

impl From<&str> for PacketData {
    fn from(s: &str) -> Self {
        match s.chars().next() {
            Some('0'..='9') => Self::Integer(s.parse().unwrap()),
            Some('[') => {
                if s.len() == 2 {
                    return Self::List(Vec::new());
                }
                let mut level = 0;
                let elements = s[1..s.len() - 1]
                    .split(|c| {
                        if c == ',' && level == 0 {
                            return true;
                        } else if c == '[' {
                            level += 1;
                        } else if c == ']' {
                            level -= 1;
                        }
                        false
                    })
                    .map(|s| s.into());
                Self::List(elements.collect())
            }
            _ => panic!("aaaarrrrrrr"),
        }
    }
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(lt), Self::Integer(rt)) => lt.cmp(rt),
            (Self::List(lt), Self::List(rt)) => {
                for (le, re) in lt.iter().zip(rt) {
                    match le.cmp(re) {
                        Ordering::Equal => continue,
                        x => return x,
                    }
                }
                lt.len().cmp(&rt.len())
            }
            (Self::Integer(_), Self::List(_)) => Self::List(vec![self.clone()]).cmp(other),
            (Self::List(_), Self::Integer(_)) => self.cmp(&Self::List(vec![other.clone()])),
        }
    }
}

macro_rules! packet {
    ([ $( $element:tt ),* ]) => {
        PacketData::List(vec![ $( packet!($element) ),* ])
    };
    ( $integer:literal ) => {
        PacketData::Integer( $integer )
    }
}

pub fn solve(input: String) {
    let mut ans1 = 0;
    let mut ans2 = 1;
    let divider_packets = [packet!([[2usize]]), packet!([[6usize]])];
    let mut packets = divider_packets.to_vec();

    for (idx, packet_pair) in input.split("\n\n").enumerate() {
        let mut lines = packet_pair.lines();
        let lp: PacketData = lines.next().unwrap().into();
        let rp: PacketData = lines.next().unwrap().into();
        if lp < rp {
            ans1 += idx + 1;
        }
        packets.push(lp);
        packets.push(rp);
    }

    packets.sort();

    for (idx, p) in packets.iter().enumerate() {
        if divider_packets.contains(p) {
            ans2 *= idx + 1;
        }
    }

    println!("ans1 = {ans1}, ans2 = {ans2}");
}
