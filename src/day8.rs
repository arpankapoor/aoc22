struct Grid {
    grid: Vec<Vec<u8>>,
}

impl From<&str> for Grid {
    fn from(grid_str: &str) -> Self {
        let grid = grid_str
            .lines()
            .map(|line| line.bytes().map(|c| c - b'0').collect())
            .collect();
        Self { grid }
    }
}

impl Grid {
    fn visible_count(&self) -> usize {
        let n = self.grid.len();
        let m = self.grid[0].len();
        let mut visible = vec![vec![false; m]; n];

        let mut lm: Vec<u8> = self.grid.iter().map(|line| line[0]).collect();
        let mut tm: Vec<u8> = self.grid[0].to_vec();
        for i in 1..n - 1 {
            for j in 1..m - 1 {
                if self.grid[i][j] > lm[i] || self.grid[i][j] > tm[j] {
                    visible[i][j] = true;
                }
                lm[i] = lm[i].max(self.grid[i][j]);
                tm[j] = tm[j].max(self.grid[i][j]);
            }
        }

        let mut rm: Vec<u8> = self.grid.iter().map(|line| line[m - 1]).collect();
        let mut dm: Vec<u8> = self.grid[n - 1].to_vec();
        for i in (1..n - 1).rev() {
            for j in (1..m - 1).rev() {
                if self.grid[i][j] > rm[i] || self.grid[i][j] > dm[j] {
                    visible[i][j] = true;
                }
                rm[i] = rm[i].max(self.grid[i][j]);
                dm[j] = dm[j].max(self.grid[i][j]);
            }
        }

        visible
            .iter()
            .map(|line| line.iter().filter(|&&e| e).count())
            .sum::<usize>()
            + 2 * (n + m - 2)
    }

    fn max_scenic_score(&self) -> usize {
        let n = self.grid.len();
        let m = self.grid[0].len();
        let mut mss = 0;
        for i in 1..n - 1 {
            for j in 1..m - 1 {
                let mut lc: usize = 0;
                let mut rc: usize = 0;
                let mut tc: usize = 0;
                let mut dc: usize = 0;
                for sj in (0..j).rev() {
                    lc += 1;
                    if self.grid[i][sj] >= self.grid[i][j] {
                        break;
                    }
                }

                for sj in j + 1..m {
                    rc += 1;
                    if self.grid[i][sj] >= self.grid[i][j] {
                        break;
                    }
                }

                for si in (0..i).rev() {
                    tc += 1;
                    if self.grid[si][j] >= self.grid[i][j] {
                        break;
                    }
                }

                for si in i + 1..n {
                    dc += 1;
                    if self.grid[si][j] >= self.grid[i][j] {
                        break;
                    }
                }

                mss = mss.max(lc * rc * tc * dc);
            }
        }
        mss
    }
}

pub fn solve(input: String) {
    let grid: Grid = input.as_str().into();
    println!("ans1 = {}, ans2 = {}", grid.visible_count(), grid.max_scenic_score());
}
