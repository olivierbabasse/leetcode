//! <https://leetcode.com/problems/path-with-maximum-gold/>

struct Solution {}

/// time-complexity : O(rows*cols)
/// space-complexity : O(1)
impl Solution {
    fn rec(grid: &mut Vec<Vec<i32>>, row: i32, col: i32) -> i32 {
        if row < 0
            || col < 0
            || row as usize >= grid.len()
            || col as usize >= grid[0].len()
            || grid[row as usize][col as usize] == 0
        {
            return 0;
        }

        const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        let val = grid[row as usize][col as usize];
        grid[row as usize][col as usize] = 0;
        let max = DIRECTIONS
            .iter()
            .map(|(r, c)| Self::rec(grid, row + r, col + c))
            .max()
            .unwrap_or_default();
        grid[row as usize][col as usize] = val;
        max + val
    }

    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                max = max.max(Self::rec(&mut grid, row as i32, col as i32));
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]),
            24
        );
        assert_eq!(
            Solution::get_maximum_gold(vec![
                vec![1, 0, 7],
                vec![2, 0, 6],
                vec![3, 4, 5],
                vec![0, 3, 0],
                vec![9, 0, 20]
            ]),
            28
        );
    }
}
