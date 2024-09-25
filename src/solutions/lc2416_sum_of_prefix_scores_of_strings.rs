//! <https://leetcode.com/problems/sum-of-prefix-scores-of-strings/>

use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

struct Solution {}

/// time-complexity : O(len(words)*len(words[x]))
/// space-complexity : O(len(words)*len(words[x]))
impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut prefix_count = HashMap::new();
        for w in &words {
            let mut hasher = DefaultHasher::new();
            for c in w.chars() {
                c.hash(&mut hasher);
                *prefix_count.entry(hasher.finish()).or_default() += 1;
            }
        }

        words
            .iter()
            .map(|w| {
                let mut count = 0;
                let mut hasher = DefaultHasher::new();
                for c in w.chars() {
                    c.hash(&mut hasher);
                    count += *prefix_count.get(&hasher.finish()).unwrap_or(&0);
                }
                count
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::sum_prefix_scores(vec!["abc".into(), "ab".into(), "bc".into(), "b".into()]),
            vec![5, 4, 3, 2]
        );
        assert_eq!(Solution::sum_prefix_scores(vec!["abcd".into()]), vec![4]);
    }
}
