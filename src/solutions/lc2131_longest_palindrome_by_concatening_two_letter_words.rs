//! <https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut freqs: HashMap<String, usize> = HashMap::new();
        let mut count = 0;
        for w in words {
            let revw = w.chars().rev().collect::<String>();

            if let Some(&f) = freqs.get(&revw) {
                *freqs.entry(w.clone()).or_default() += 1;

                if f >= 1 {
                    *freqs.get_mut(&w).unwrap() -= 1;
                    *freqs.get_mut(&revw).unwrap() -= 1;
                    count += 4;
                }
            } else {
                *freqs.entry(w.clone()).or_default() += 1;
            }
        }
        for (f, c) in freqs {
            if f == f.chars().rev().collect::<String>() && c > 0 {
                count += 2;
                break;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::longest_palindrome(vec!["lc".into(), "cl".into(), "gg".into()]),
            6
        );
        assert_eq!(
            Solution::longest_palindrome(vec![
                "ab".into(),
                "ty".into(),
                "yt".into(),
                "lc".into(),
                "cl".into(),
                "ab".into()
            ]),
            8
        );
        assert_eq!(
            Solution::longest_palindrome(vec!["cc".into(), "ll".into(), "xx".into()]),
            2
        );
    }
}
