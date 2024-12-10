//! <https://leetcode.com/problems/find-longest-special-substring-that-occurs-thrice-i/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n^2)
impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut freqs: HashMap<(u8, usize), usize> = HashMap::new();
        let s = s.as_bytes();
        for start in 0..s.len() {
            for end in start..s.len() {
                if s[start] == s[end] {
                    *freqs.entry((s[start], end - start + 1)).or_default() += 1;
                } else {
                    break;
                }
            }
        }

        freqs
            .into_iter()
            .filter(|(_, v)| *v >= 3)
            .map(|(k, _)| k)
            .max_by(|x, y| x.1.cmp(&y.1))
            .map(|k| k.1 as i32)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::maximum_length("aaaa".into()), 2);
        assert_eq!(Solution::maximum_length("abcdef".into()), -1);
        assert_eq!(Solution::maximum_length("abcaba".into()), 1);
    }
}
