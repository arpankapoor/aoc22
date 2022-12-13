use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Debug)]
struct Vertex {
    row: usize,
    col: usize,
    val: u8,
}

struct Graph {
    adj_list: HashMap<Vertex, Vec<Vertex>>,
    src: Vertex,
    dst: Vertex,
}

impl From<&str> for Graph {
    fn from(s: &str) -> Self {
        let grid: Vec<_> = s.lines().map(|line| line.as_bytes().to_vec()).collect();
        let nrows = grid.len();
        let ncols = grid[0].len();
        let mut src = Vertex::default();
        let mut dst = Vertex::default();
        let mut adj_list = HashMap::new();

        for row in 0..nrows {
            for col in 0..ncols {
                let value = grid[row][col];
                let vertex = Vertex {
                    row,
                    col,
                    val: match value {
                        b'S' => b'a',
                        b'E' => b'z',
                        v => v,
                    },
                };

                if value == b'S' {
                    src = vertex;
                } else if value == b'E' {
                    dst = vertex;
                }

                let mut neighbors = Vec::new();
                for (dr, dc) in [(-1isize, 0), (1, 0), (0, -1isize), (0, 1)] {
                    let neighbor_row = vertex.row.checked_add_signed(dr);
                    let neighbor_col = vertex.col.checked_add_signed(dc);
                    let neighbor = match (neighbor_row, neighbor_col) {
                        (Some(nr), Some(nc)) if nr < nrows && nc < ncols => Vertex {
                            row: nr,
                            col: nc,
                            val: match grid[nr][nc] {
                                b'S' => b'a',
                                b'E' => b'z',
                                v => v,
                            },
                        },
                        _ => continue,
                    };

                    // neighbor value should be atmost value + 1
                    if neighbor.val > vertex.val + 1 {
                        continue;
                    }
                    neighbors.push(neighbor);
                }

                if !neighbors.is_empty() {
                    adj_list.insert(vertex, neighbors);
                }
            }
        }
        Self { adj_list, src, dst }
    }
}

struct Visit<V> {
    vertex: V,
    distance: usize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}

impl Graph {
    fn shortest_path(&self, start: Option<Vertex>) -> Option<usize> {
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut to_visit = BinaryHeap::new();

        let start = start.unwrap_or(self.src);

        distances.insert(start, 0);
        to_visit.push(Visit {
            vertex: start,
            distance: 0,
        });

        while let Some(Visit { vertex, distance }) = to_visit.pop() {
            if vertex == self.dst {
                break;
            }

            if !visited.insert(vertex) {
                // already visited
                continue;
            }

            if let Some(neighbors) = self.adj_list.get(&vertex) {
                for neighbor in neighbors {
                    let new_distance = distance + 1;
                    let is_shorter = distances
                        .get(neighbor)
                        .map_or(true, |&current| new_distance < current);

                    if is_shorter {
                        distances.insert(*neighbor, new_distance);
                        to_visit.push(Visit {
                            vertex: *neighbor,
                            distance: new_distance,
                        });
                    }
                }
            }
        }

        distances.get(&self.dst).copied()
    }
}

pub fn solve(input: String) {
    let grid: Graph = input.as_str().into();
    let ans1 = grid.shortest_path(None).unwrap();
    let ans2 = grid
        .adj_list
        .keys()
        .filter(|&&v| v.val == b'a')
        .flat_map(|&v| grid.shortest_path(Some(v)))
        .min()
        .unwrap();
    println!("ans1 = {ans1}, ans2 = {ans2}");
}
