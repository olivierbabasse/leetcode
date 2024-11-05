//! <https://leetcode.com/problems/minimum-number-of-changes-to-make-binary-string-beautiful/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes()
            .chunks_exact(2)
            .fold(0, |acc, x| acc + if x[0] != x[1] { 1 } else { 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_changes("1001".into()), 2);
        assert_eq!(Solution::min_changes("10".into()), 1);
        assert_eq!(Solution::min_changes("0000".into()), 0);
        assert_eq!(Solution::min_changes("11000111".into()), 1);
    }
}
