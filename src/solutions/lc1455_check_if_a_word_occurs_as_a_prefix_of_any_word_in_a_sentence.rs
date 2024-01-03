//! <https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence
            .split(' ')
            .position(|s| (*s).starts_with(&search_word))
            .map(|index| index as i32 + 1)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::is_prefix_of_word("i love eating burger".into(), "burg".into()),
            4
        );
        assert_eq!(
            Solution::is_prefix_of_word("i am tired".into(), "you".into()),
            -1
        );
    }
}
