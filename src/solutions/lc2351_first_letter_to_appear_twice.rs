//! <https://leetcode.com/problems/first-letter-to-appear-twice/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut seen = [false; 26];
        for c in s.chars() {
            let index = (c as u32 - 'a' as u32) as usize;
            if seen[index] {
                return c;
            } else {
                seen[index] = true;
            }
        }
        'a'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::repeated_character("abccbaacz".into()), 'c');
        assert_eq!(Solution::repeated_character("abcdd".into()), 'd');
    }
}
