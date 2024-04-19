//! <https://leetcode.com/problems/max-area-of-island/>

struct Solution {}

/// time-complexity : O(n*m)
/// space-complexity : O(n*m)
impl Solution {
    fn visit(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> i32 {
        if visited[i][j] || grid[i][j] != 1 {
            return 0;
        }

        let rows = grid.len();
        let cols = grid[0].len();
        visited[i][j] = true;
        let mut area = 1;
        if i > 0 {
            area += Self::visit(grid, visited, i - 1, j)
        }
        if i < rows - 1 {
            area += Self::visit(grid, visited, i + 1, j)
        }
        if j > 0 {
            area += Self::visit(grid, visited, i, j - 1)
        }
        if j < cols - 1 {
            area += Self::visit(grid, visited, i, j + 1)
        }
        area
    }

    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut max = 0;
        for i in 0..rows {
            for j in 0..cols {
                if !visited[i][j] && grid[i][j] == 1 {
                    max = max.max(Self::visit(&grid, &mut visited, i, j));
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        );
    }
}
