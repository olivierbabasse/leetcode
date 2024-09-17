//! <https://leetcode.com/problems/uncommon-words-from-two-sentences/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        s1.split_ascii_whitespace()
            .chain(s2.split_ascii_whitespace())
            .fold(HashMap::<&str, usize>::new(), |mut freqs, word| {
                *freqs.entry(word).or_default() += 1;
                freqs
            })
            .into_iter()
            .filter(|(_, f)| *f == 1)
            .map(|(w, _)| w.to_string())
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::array_eq;

    use super::*;

    #[test]
    fn test_cases() {
        assert!(array_eq(
            &Solution::uncommon_from_sentences(
                "this apple is sweet".into(),
                "this apple is sour".into()
            ),
            &["sweet".to_string(), "sour".to_string()]
        ))
    }
}
