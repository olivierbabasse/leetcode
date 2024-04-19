//! <https://leetcode.com/problems/number-of-islands/>

struct Solution {}

/// time-complexity : O(n*m)
/// space-complexity : O(n*m)
impl Solution {
    fn visit(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
        if visited[i][j] || grid[i][j] != '1' {
            return;
        }

        let rows = grid.len();
        let cols = grid[0].len();
        visited[i][j] = true;
        if i > 0 {
            Self::visit(grid, visited, i - 1, j)
        }
        if i < rows - 1 {
            Self::visit(grid, visited, i + 1, j)
        }
        if j > 0 {
            Self::visit(grid, visited, i, j - 1)
        }
        if j < cols - 1 {
            Self::visit(grid, visited, i, j + 1)
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut total = 0;
        for i in 0..rows {
            for j in 0..cols {
                if !visited[i][j] && grid[i][j] == '1' {
                    total += 1;
                    Self::visit(&grid, &mut visited, i, j);
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
