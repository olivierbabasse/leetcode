//! <https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut freqs = [0; 26];
        s.as_bytes()
            .iter()
            .for_each(|b| freqs[(b - b'a') as usize] += 1);
        t.as_bytes()
            .iter()
            .for_each(|b| freqs[(b - b'a') as usize] -= 1);
        freqs.into_iter().filter(|&v| v > 0).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_steps("bab".into(), "aba".into()), 1);
        assert_eq!(Solution::min_steps("leetcode".into(), "practice".into()), 5);
        assert_eq!(Solution::min_steps("anagram".into(), "mangaar".into()), 0);
    }
}
