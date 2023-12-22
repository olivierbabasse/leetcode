//! <https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(len(s1)*len(s2))
/// space-complexity : O(len(s1)*len(s2))
impl Solution {
    fn rec(
        s1: &[u8],
        index1: usize,
        s2: &[u8],
        index2: usize,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(val) = cache.get(&(index1, index2)) {
            *val
        } else if index1 >= s1.len() {
            s2[index2..].iter().map(|&c| c as i32).sum()
        } else if index2 >= s2.len() {
            s1[index1..].iter().map(|&c| c as i32).sum()
        } else if s1[index1] == s2[index2] {
            Self::rec(s1, index1 + 1, s2, index2 + 1, cache)
        } else {
            let res = i32::min(
                s1[index1] as i32 + Self::rec(s1, index1 + 1, s2, index2, cache),
                s2[index2] as i32 + Self::rec(s1, index1, s2, index2 + 1, cache),
            );
            cache.insert((index1, index2), res);
            res
        }
    }

    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        Self::rec(s1.as_bytes(), 0, s2.as_bytes(), 0, &mut HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::minimum_delete_sum("sea".into(), "eat".into()),
            231
        );
        assert_eq!(
            Solution::minimum_delete_sum("delete".into(), "leet".into()),
            403
        );
    }
}
