//! <https://leetcode.com/problems/number-of-wonderful-substrings/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let len = word.len();
        let mut mask = 0;
        let mut res = 0;

        let mut freq = HashMap::new();
        freq.insert(0, 1);

        for i in 0..len {
            let bit = word.as_bytes()[i] - b'a';
            mask ^= 1 << bit;

            let f = freq.get(&mask).unwrap_or(&0);
            res += f;
            freq.insert(mask, f + 1);

            for j in 0..10 {
                res += freq.get(&(mask ^ 1 << j)).unwrap_or(&0);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::wonderful_substrings("aba".into()), 4);
        assert_eq!(Solution::wonderful_substrings("aabb".into()), 9);
    }
}
