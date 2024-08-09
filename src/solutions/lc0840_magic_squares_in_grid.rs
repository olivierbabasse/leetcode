//! <https://leetcode.com/problems/magic-squares-in-grid/>

struct Solution {}

/// time-complexity : O(rows*cols)
/// space-complexity : O(1)
impl Solution {
    fn is_magic_square(grid: &[Vec<i32>], row: usize, col: usize) -> bool {
        let mut seen = [false; 10];
        for i in 0..3 {
            for j in 0..3 {
                let n = grid[row + i][col + j];
                if !(1..=9).contains(&n) || seen[n as usize] {
                    return false;
                }
                seen[n as usize] = true;
            }
        }

        let d1 = grid[row][col] + grid[row + 1][col + 1] + grid[row + 2][col + 2];
        let d2 = grid[row + 2][col] + grid[row + 1][col + 1] + grid[row][col + 2];
        let r1 = grid[row][col] + grid[row][col + 1] + grid[row][col + 2];
        let r2 = grid[row + 1][col] + grid[row + 1][col + 1] + grid[row + 1][col + 2];
        let r3 = grid[row + 2][col] + grid[row + 2][col + 1] + grid[row + 2][col + 2];
        let c1 = grid[row][col] + grid[row + 1][col] + grid[row + 2][col];
        let c2 = grid[row][col + 1] + grid[row + 1][col + 1] + grid[row + 2][col + 1];
        let c3 = grid[row][col + 2] + grid[row + 1][col + 2] + grid[row + 2][col + 2];

        d1 == d2 && d2 == r1 && r1 == r2 && r2 == r3 && r3 == c1 && c1 == c2 && c2 == c3
    }
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() <= 2 || grid[0].len() <= 2 {
            return 0;
        }
        let mut res = 0;
        for row in 0..grid.len() - 2 {
            for col in 0..grid[0].len() - 2 {
                if Self::is_magic_square(&grid, row, col) {
                    res += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::num_magic_squares_inside(vec![
                vec![4, 3, 8, 4],
                vec![9, 5, 1, 9],
                vec![2, 7, 6, 2]
            ]),
            1
        );
    }
}
