//! <https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/>

use std::collections::HashSet;

struct Solution {}

/// time-complexity : O(2^n)
/// space-complexity : O(n)
impl Solution {
    fn rec<'a>(s: &'a str, offset: usize, set: &mut HashSet<&'a str>) -> i32 {
        let mut max = 0;
        if offset < s.len() {
            for i in offset + 1..=s.len() {
                let substring = &s[offset..i];
                if !set.contains(substring) {
                    set.insert(substring);
                    max = max.max(Self::rec(s, i, set) + 1);
                    set.remove(substring);
                }
            }
        }
        max
    }

    pub fn max_unique_split(s: String) -> i32 {
        Self::rec(&s, 0, &mut HashSet::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_unique_split("ababccc".into()), 5);
        assert_eq!(Solution::max_unique_split("aba".into()), 2);
        assert_eq!(Solution::max_unique_split("aa".into()), 1);
    }
}
