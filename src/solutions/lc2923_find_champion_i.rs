//! <https://leetcode.com/problems/find-champion-i/>

struct Solution {}

/// time-complexity : O(rows)
/// space-complexity : O(1)
impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let mut index = 0;
        for (r, row) in grid.iter().enumerate() {
            if row[index] == 1 {
                index = r
            }
        }
        index as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_champion(vec![vec![0, 1], vec![0, 0]]), 0);
        assert_eq!(
            Solution::find_champion(vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]]),
            1
        );
    }
}
