use std::collections::HashSet;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let num_lines = grid.len();
        let num_cols = grid[0].len();
        let mut islands = 0;
        // let mut seen: HashSet<(isize,isize)> = HashSet::new();gc
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
        for i in 0..num_lines {
            for j in 0..num_cols {
                if grid[i][j] == '1' && !seen[i][j] {
                    islands += 1;
                    Self::dfs(&grid, &mut seen, i as isize, j as isize);
                }
            }
        }
        islands
    }

    fn dfs(grid: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>, i: isize, j: isize) {
        let num_lines = grid.len() as isize;
        let num_cols = grid[0].len() as isize;

        seen[i as usize][j as usize] = true;

        let nexts = [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)];
        for (ni, nj) in nexts {
            if ni < 0
                || ni >= num_lines
                || nj < 0
                || nj >= num_cols
                || grid[ni as usize][nj as usize] == '0'
                || seen[ni as usize][nj as usize]
            {
                continue;
            }
            Self::dfs(grid, seen, ni, nj);
        }
    }
}
