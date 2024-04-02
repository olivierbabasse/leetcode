//! <https://leetcode.com/problems/length-of-last-word/>

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split(' ')
            .filter(|&s| !s.is_empty())
            .last()
            .unwrap_or_default()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::length_of_last_word("Hello World".into()), 5);
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".into()),
            4
        );
    }
}
