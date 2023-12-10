//! <https://leetcode.com/problems/transpose-matrix/description/>

struct Solution {}

/// time-complexity : O(n*m)
/// space-complexity : O(n*m)
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dest = vec![vec![0; rows]; cols];
        for (i, row) in matrix.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                dest[j][i] = val;
            }
        }
        dest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        );
    }
}
