//! <https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1
            .iter()
            .flat_map(|w| w.chars())
            .eq(word2.iter().flat_map(|w| w.chars()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::array_strings_are_equal(
            vec!["ab".into(), "c".into()],
            vec!["a".into(), "bc".into()]
        ));
        assert!(!Solution::array_strings_are_equal(
            vec!["ac".into(), "b".into()],
            vec!["a".into(), "bc".into()]
        ));
    }
}
