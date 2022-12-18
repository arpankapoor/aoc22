use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node {
    name: String,
    rate: i32,
    mask: i64,
    connected_nodes: HashSet<String>,
}

impl Node {
    fn from_line(line: &str, idx: usize) -> Self {
        let mut split = line.split(';');
        let mut p1 = split.next().unwrap().split_ascii_whitespace();
        let name = p1.nth(1).unwrap().to_owned();
        let rate = p1
            .nth(2)
            .unwrap()
            .split('=')
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let p2 = &split.next().unwrap().split("valve").last().unwrap()[1..].trim_start();
        let cnodes = p2.split(", ").map(|c| c.to_owned()).collect();
        Self {
            name,
            rate,
            mask: 1 << idx,
            connected_nodes: cnodes,
        }
    }
}

fn floyd_warshall(nodes: &HashMap<String, Node>) -> HashMap<String, HashMap<String, i32>> {
    let mut dist = HashMap::<String, HashMap<String, i32>>::new();

    for (name, node) in nodes {
        dist.insert(
            name.clone(),
            nodes
                .keys()
                .map(|n| {
                    (
                        n.clone(),
                        if node.connected_nodes.contains(n) {
                            1
                        } else if n == name {
                            0
                        } else {
                            i32::MAX
                        },
                    )
                })
                .collect(),
        );
    }

    for k in nodes.values() {
        for i in nodes.values() {
            for j in nodes.values() {
                let with_k = dist[&i.name][&k.name].saturating_add(dist[&k.name][&j.name]);
                dist.get_mut(&i.name)
                    .map(|m| m.get_mut(&j.name).map(|val| *val = with_k.min(*val)));
            }
        }
    }
    dist
}

fn visit(
    start_node: &Node,
    nodes: &HashMap<String, Node>,
    distances: &HashMap<String, HashMap<String, i32>>,
    budget: i32,
    state: i64,
    flow: i32,
    ans: &mut HashMap<i64, i32>,
) {
    ans.entry(state)
        .and_modify(|old| *old = flow.max(*old))
        .or_insert(0);
    for end_node in nodes.values().filter(|n| n.rate > 0) {
        let new_budget = budget - distances[&start_node.name][&end_node.name] - 1;
        if (state & end_node.mask) != 0 || new_budget <= 0 {
            continue;
        }
        visit(
            end_node,
            nodes,
            distances,
            new_budget,
            state | end_node.mask,
            flow + new_budget * end_node.rate,
            ans,
        )
    }
}

pub fn solve(input: String) {
    let mut nodes = HashMap::new();
    for (idx, line) in input.lines().enumerate() {
        let node = Node::from_line(line, idx);
        nodes.insert(node.name.clone(), node);
    }

    let distances = floyd_warshall(&nodes);

    let mut ans1_map = HashMap::new();
    visit(&nodes["AA"], &nodes, &distances, 30, 0, 0, &mut ans1_map);
    let ans1 = ans1_map.values().max().unwrap();

    let mut ans2_map = HashMap::new();
    visit(&nodes["AA"], &nodes, &distances, 26, 0, 0, &mut ans2_map);
    let ans2 = ans2_map
        .iter()
        .flat_map(|(&k1, &v1)| {
            ans2_map
                .iter()
                .filter_map(move |(&k2, v2)| if k1 & k2 == 0 { Some(v1 + v2) } else { None })
        })
        .max()
        .unwrap();

    println!("ans1 = {ans1}, ans2 = {ans2}");
}
