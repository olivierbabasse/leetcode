//! <https://leetcode.com/problems/valid-anagram/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut schars: HashMap<char, usize> = HashMap::new();
        let mut tchars: HashMap<char, usize> = HashMap::new();
        s.chars().for_each(|c| *schars.entry(c).or_default() += 1);
        t.chars().for_each(|c| *tchars.entry(c).or_default() += 1);
        schars == tchars
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::is_anagram("anagram".into(), "nagaram".into()));
        assert!(!Solution::is_anagram("rat".into(), "car".into()));
    }
}
