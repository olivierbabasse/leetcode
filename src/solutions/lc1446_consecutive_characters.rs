//! <https://leetcode.com/problems/consecutive-characters/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut current_char = s.chars().nth(0).unwrap();
        let mut current_count = 1;
        let mut max_count = 0;
        for c in s.chars().skip(1) {
            if c != current_char {
                max_count = max_count.max(current_count);
                current_char = c;
                current_count = 0;
            }
            current_count += 1;
        }
        max_count.max(current_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_power("leetcode".into()), 2);
        assert_eq!(Solution::max_power("abbcccddddeeeeedcba".into()), 5);
    }
}
