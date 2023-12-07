//! <https://leetcode.com/problems/sum-of-digits-of-string-after-convert/>

struct Solution {}

/// time-complexity : O(k*len(s))
/// space-complexity : O(len(s))
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut s: String = s
            .chars()
            .map(|c| (c as u8 - b'a' + 1).to_string())
            .collect();
        let mut res = 0;
        for _ in 0..k {
            res = s.chars().map(|c| (c as u8 - b'0') as i32).sum();
            s = res.to_string();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::get_lucky("iiii".into(), 1), 36);
        assert_eq!(Solution::get_lucky("leetcode".into(), 2), 6);
        assert_eq!(Solution::get_lucky("zbax".into(), 2), 8);
    }
}
