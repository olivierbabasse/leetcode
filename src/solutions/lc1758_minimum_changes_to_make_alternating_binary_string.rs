//! <https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut s1 = String::new();
        let mut s2 = String::new();
        for i in 0..s.len() {
            if i % 2 == 0 {
                s1.push('0');
                s2.push('1');
            } else {
                s1.push('1');
                s2.push('0');
            }
        }
        let c1 = s.chars().zip(s1.chars()).filter(|(a, b)| a != b).count();
        let c2 = s.chars().zip(s2.chars()).filter(|(a, b)| a != b).count();
        c1.min(c2) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_operations("0100".into()), 1);
        assert_eq!(Solution::min_operations("10".into()), 0);
        assert_eq!(Solution::min_operations("1111".into()), 2);
    }
}
