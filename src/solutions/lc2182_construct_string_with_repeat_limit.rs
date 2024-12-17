//! <https://leetcode.com/problems/construct-string-with-repeat-limit/>

use std::collections::{BinaryHeap, HashMap};

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let freqs = s.as_bytes().iter().fold(HashMap::new(), |mut acc, &b| {
            *acc.entry(b).or_insert(0) += 1;
            acc
        });
        let mut chars: BinaryHeap<_> = freqs.into_iter().collect();
        let mut res = String::new();

        while !chars.is_empty() {
            let (ch, count) = chars.pop().unwrap();
            let n = count.min(repeat_limit);
            (0..n).for_each(|_| res.push(ch as char));

            if count > n && !chars.is_empty() {
                let (nextch, nextcount) = chars.pop().unwrap();
                res.push(nextch as char);
                if nextcount > 1 {
                    chars.push((nextch, nextcount - 1));
                }
                chars.push((ch, count - n));
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::repeat_limited_string("cczazcc".into(), 3),
            "zzcccac".to_string()
        );
        assert_eq!(
            Solution::repeat_limited_string("aababab".into(), 2),
            "bbabaa".to_string()
        );
    }
}
