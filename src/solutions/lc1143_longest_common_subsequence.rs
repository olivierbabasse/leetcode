//! <https://leetcode.com/problems/longest-common-subsequence/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(len(t1)*len(t2))
/// space-complexity : O(len(t1)*len(t2))
impl Solution {
    fn lcs(
        text1: &str,
        index1: i32,
        text2: &str,
        index2: i32,
        cache: &mut HashMap<(i32, i32), i32>,
    ) -> i32 {
        if index1 < 0 || index2 < 0 {
            0
        } else if let Some(res) = cache.get(&(index1, index2)) {
            *res
        } else if text1.as_bytes()[index1 as usize] == text2.as_bytes()[index2 as usize] {
            Self::lcs(text1, index1 - 1, text2, index2 - 1, cache) + 1
        } else {
            let res = i32::max(
                Self::lcs(text1, index1 - 1, text2, index2, cache),
                Self::lcs(text1, index1, text2, index2 - 1, cache),
            );
            cache.insert((index1, index2), res);
            res
        }
    }

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut cache = HashMap::new();
        Self::lcs(
            &text1,
            (text1.len() - 1) as i32,
            &text2,
            (text2.len() - 1) as i32,
            &mut cache,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".into(), "ace".into()),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".into(), "abc".into()),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".into(), "def".into()),
            0
        );
        assert_eq!(
            Solution::longest_common_subsequence(
                "pmjghexybyrgzczy".into(),
                "hafcdqbgncrcbihkd".into()
            ),
            4
        );
    }
}
