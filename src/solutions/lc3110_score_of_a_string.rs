//! <https://leetcode.com/problems/score-of-a-string/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .map(|w| (w[0] as i32).abs_diff(w[1] as i32))
            .sum::<u32>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::score_of_string("hello".into()), 13);
        assert_eq!(Solution::score_of_string("zaz".into()), 50);
    }
}
