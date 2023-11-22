//! <https://leetcode.com/problems/diagonal-traverse-ii/>

struct Solution {}

/// time-complexity : O(n*m)
/// space-complexity : O(n*m.log(n*m))
impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut positions = Vec::new();
        for (i, v) in nums.iter().enumerate() {
            for (j, _) in v.iter().enumerate() {
                positions.push((i + j, j));
            }
        }
        let mut nnums = nums.into_iter().flatten().enumerate().collect::<Vec<_>>();
        nnums.sort_unstable_by_key(|(index, _)| positions[*index]);
        nnums.iter().map(|(_, num)| *num).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 4, 2, 7, 5, 3, 8, 6, 9]
        );
        assert_eq!(
            Solution::find_diagonal_order(vec![
                vec![1, 2, 3, 4, 5],
                vec![6, 7],
                vec![8],
                vec![9, 10, 11],
                vec![12, 13, 14, 15, 16]
            ]),
            vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16]
        );
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9]]),
            [1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }
}
