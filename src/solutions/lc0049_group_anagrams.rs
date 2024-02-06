//! <https://leetcode.com/problems/group-anagrams/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let sorted_strs = strs
            .iter()
            .map(|s| {
                let mut s = s.bytes().collect::<Vec<_>>();
                s.sort_unstable();
                std::str::from_utf8(&s).unwrap().to_string()
            })
            .collect::<Vec<_>>();

        let mut map = HashMap::<&str, Vec<String>>::new();
        for i in 0..strs.len() {
            map.entry(&sorted_strs[i])
                .or_default()
                .push(strs[i].clone());
        }
        map.values().cloned().collect()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::group_anagrams(vec![
                "eat".into(),
                "tea".into(),
                "tan".into(),
                "ate".into(),
                "nat".into(),
                "bat".into()
            ]),
            vec![
                vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
                vec!["nat".to_string(), "tan".to_string()],
                vec!["bat".to_string()],
            ]
        );
    }
}
*/
