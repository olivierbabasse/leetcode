//! <https://leetcode.com/problems/longest-substring-without-repeating-characters/>

use std::collections::VecDeque;

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n)
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut deque = VecDeque::new();
        let mut max_len = 0;
        for c in s.chars() {
            while deque.contains(&c) {
                deque.pop_front();
            }
            deque.push_back(c);
            max_len = max_len.max(deque.len());
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
    }
}
