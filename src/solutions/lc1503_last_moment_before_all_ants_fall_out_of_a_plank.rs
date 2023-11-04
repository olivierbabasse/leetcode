//! <https://leetcode.com/problems/last-moment-before-all-ants-fall-out-of-a-plank/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        left.into_iter().max().unwrap_or_default().max(
            right
                .into_iter()
                .min()
                .map(|val| n - val)
                .unwrap_or_default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::get_last_moment(4, vec![4, 3], vec![0, 1]), 4);
        assert_eq!(
            Solution::get_last_moment(7, vec![], vec![0, 1, 2, 3, 4, 5, 6, 7]),
            7
        );
        assert_eq!(
            Solution::get_last_moment(7, vec![0, 1, 2, 3, 4, 5, 6, 7], vec![]),
            7
        );
    }
}
