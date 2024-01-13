//! <https://leetcode.com/problems/longest-palindrome/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut freqs: HashMap<u8, usize> = HashMap::new();
        let mut count: i32 = 0;
        s.as_bytes().iter().for_each(|&b| {
            *freqs.entry(b).or_default() += 1;
            if freqs[&b] % 2 == 0 {
                count += 2;
            }
        });
        if s.len() as i32 > count {
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::longest_palindrome("abccccdd".into()), 7);
        assert_eq!(Solution::longest_palindrome("a".into()), 1);
    }
}
