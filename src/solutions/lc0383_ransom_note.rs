//! <https://leetcode.com/problems/ransom-note/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut letters = HashMap::new();
        magazine.chars().for_each(|l| {
            letters.entry(l).and_modify(|c| *c += 1).or_insert(1);
        });

        ransom_note.chars().for_each(|l| {
            letters.entry(l).and_modify(|c| *c -= 1).or_insert(-1);
        });

        letters.iter().all(|(_, v)| v >= &0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert!(!Solution::can_construct("a".into(), "b".into()));
        assert!(!Solution::can_construct("aa".into(), "ab".into()));
        assert!(Solution::can_construct("aa".into(), "aab".into()));
    }
}
