//! <https://leetcode.com/problems/reverse-words-in-a-string-iii/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .map(|s| s.chars().rev().collect())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".into()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
        assert_eq!(
            Solution::reverse_words("God Ding".into()),
            "doG gniD".to_string()
        );
    }
}
