//! <https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut f: HashMap<char, usize> = HashMap::new();
        for l in words.iter().flat_map(|s| s.chars()) {
            *f.entry(l).or_default() += 1;
        }
        f.values().all(|v| v % words.len() == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::make_equal(vec![
            "abc".into(),
            "aabc".into(),
            "bc".into()
        ]));
        assert!(!Solution::make_equal(vec!["ab".into(), "a".into()]));
    }
}
