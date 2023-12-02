//! <https://leetcode.com/problems/triangle/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n)
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut prev = vec![0; n];
        let mut curr = vec![0; n];
        prev[..n].copy_from_slice(&triangle[n - 1][..n]);
        if n >= 2 {
            for i in (0..=n - 2).rev() {
                for j in (0..=i).rev() {
                    curr[j] = triangle[i][j] + i32::min(prev[j], prev[j + 1]);
                }
                prev = curr.clone();
            }
        }
        prev[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
    }
}
