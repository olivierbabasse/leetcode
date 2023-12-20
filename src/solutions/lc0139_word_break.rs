//! <https://leetcode.com/problems/word-break/>

use std::collections::HashMap;
use std::collections::HashSet;

struct Solution {}

/// time-complexity : O(n*len(dict)*len(word))
/// space-complexity : O(n)
impl Solution {
    fn rec(s: &str, pos: usize, words: &HashSet<&str>, cache: &mut HashMap<usize, bool>) -> bool {
        let len = s.len();
        if pos == len {
            true
        } else if let Some(res) = cache.get(&pos) {
            *res
        } else {
            let mut ret = false;
            for i in pos..len {
                if words.contains(&s[pos..=i]) && Self::rec(s, i + 1, words, cache) {
                    ret = true;
                }
            }
            cache.insert(pos, ret);
            ret
        }
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let words: HashSet<&str> = word_dict.iter().map(AsRef::as_ref).collect();
        let mut cache = HashMap::new();
        Self::rec(&s, 0, &words, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::word_break(
            "leetcode".into(),
            vec!["leet".into(), "code".into()]
        ));
        assert!(Solution::word_break(
            "applepenapple".into(),
            vec!["apple".into(), "pen".into()]
        ));
        assert!(!Solution::word_break(
            "catsandog".into(),
            vec![
                "cats".into(),
                "dog".into(),
                "sand".into(),
                "and".into(),
                "cat".into()
            ]
        ));
        assert!(!Solution::word_break(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".into(),
            vec!["a".into(),"aa".into(),"aaa".into(),"aaaa".into(),"aaaaa".into(),"aaaaaa".into(),"aaaaaaa".into(),"aaaaaaaa".into(),"aaaaaaaaa".into(),"aaaaaaaaaa".into()]
        ));
    }
}
