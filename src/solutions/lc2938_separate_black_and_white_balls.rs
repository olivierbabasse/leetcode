//! <https://leetcode.com/problems/separate-black-and-white-balls/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let (mut res, mut blacks) = (0, 0);
        for b in s.as_bytes() {
            if b == &b'0' {
                res += blacks;
            } else {
                blacks += 1;
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
        assert_eq!(Solution::minimum_steps("101".into()), 1);
        assert_eq!(Solution::minimum_steps("100".into()), 2);
        assert_eq!(Solution::minimum_steps("0111".into()), 0);
    }
}
