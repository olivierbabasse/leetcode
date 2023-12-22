//! <https://leetcode.com/problems/maximum-score-after-splitting-a-string/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let len = s.len();
        let s = s.as_bytes();
        let ones = s.iter().filter(|&&c| c == b'1').count();
        let mut score_zero = if s[0] == b'0' { 1 } else { 0 };
        let mut score_one = ones - if s[0] == b'1' { 1 } else { 0 };
        let mut max = score_zero + score_one;
        for &c in s.iter().take(len - 1).skip(1) {
            if c == b'1' {
                score_one -= 1;
            } else {
                score_zero += 1;
            }
            max = max.max(score_zero + score_one);
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_score("011101".into()), 5);
        assert_eq!(Solution::max_score("00111".into()), 5);
        assert_eq!(Solution::max_score("1111".into()), 3);
        assert_eq!(Solution::max_score("00".into()), 1);
    }
}
