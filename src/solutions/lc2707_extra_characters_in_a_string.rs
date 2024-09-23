//! <https://leetcode.com/problems/extra-characters-in-a-string/>

use std::collections::{HashMap, HashSet};

struct Solution {}

/// time-complexity : O(n^3)
/// space-complexity : O(n)
impl Solution {
    fn rec(s: &str, offset: usize, dic: &HashSet<&str>, cache: &mut HashMap<usize, i32>) -> i32 {
        let len = s.len();
        if offset == len {
            return 0;
        }
        if let Some(res) = cache.get(&offset) {
            return *res;
        }

        let mut res = Self::rec(s, offset + 1, dic, cache) + 1;
        for i in offset..len {
            let cur = &s[offset..i + 1];
            if dic.contains(cur) {
                res = res.min(Self::rec(s, i + 1, dic, cache));
            }
        }

        cache.insert(offset, res);
        res
    }

    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let dic = dictionary
            .iter()
            .map(|s| s.as_ref())
            .collect::<HashSet<_>>();
        Self::rec(&s, 0, &dic, &mut HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::min_extra_char(
                "leetscode".into(),
                vec![
                    "leet".to_string(),
                    "code".to_string(),
                    "leetcode".to_string()
                ]
            ),
            1
        );
        assert_eq!(
            Solution::min_extra_char(
                "sayhelloworld".into(),
                vec!["hello".to_string(), "world".to_string()]
            ),
            3
        );
    }
}
