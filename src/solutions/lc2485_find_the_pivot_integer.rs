//! <https://leetcode.com/problems/find-the-pivot-integer/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let sum = n * (n + 1) / 2;
        let pivot = (sum as f64).sqrt() as i32;
        if pivot * pivot == sum {
            pivot
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::pivot_integer(8), 6);
        assert_eq!(Solution::pivot_integer(1), 1);
        assert_eq!(Solution::pivot_integer(4), -1);
    }
}
