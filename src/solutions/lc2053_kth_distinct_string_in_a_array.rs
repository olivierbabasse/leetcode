//! <https://leetcode.com/problems/kth-distinct-string-in-an-array/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut counts: HashMap<_, usize> = HashMap::new();
        for a in &arr {
            *counts.entry(a).or_default() += 1;
        }
        let mut count = 0;
        for a in &arr {
            if *counts.get(a).unwrap() == 1 {
                count += 1;
                if count == k {
                    return a.clone();
                }
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::kth_distinct(
                vec![
                    "d".into(),
                    "b".into(),
                    "c".into(),
                    "b".into(),
                    "c".into(),
                    "a".into()
                ],
                2
            ),
            "a".to_string()
        );
        assert_eq!(
            Solution::kth_distinct(vec!["aaa".into(), "aa".into(), "a".into(),], 1),
            "aaa".to_string()
        );
        assert_eq!(
            Solution::kth_distinct(vec!["a".into(), "b".into(), "a".into(),], 3),
            "".to_string()
        );
    }
}
