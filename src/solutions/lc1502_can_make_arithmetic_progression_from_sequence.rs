//! <https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        (0..arr.len() - 2).all(|i| arr[i + 2] - arr[i + 1] == arr[i + 1] - arr[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::can_make_arithmetic_progression(vec![3, 5, 1]));
        assert!(!Solution::can_make_arithmetic_progression(vec![1, 2, 4]));
    }
}
