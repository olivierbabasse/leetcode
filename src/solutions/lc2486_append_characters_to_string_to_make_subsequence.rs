//! <https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let (mut first, mut max) = (0, 0);
        while first < s.len() && max < t.len() {
            if s.as_bytes()[first] == t.as_bytes()[max] {
                max += 1;
            }
            first += 1;
        }
        (t.len() - max) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::append_characters("coaching".into(), "coding".into()),
            4
        );
        assert_eq!(Solution::append_characters("abcde".into(), "a".into()), 0);
        assert_eq!(Solution::append_characters("z".into(), "abcde".into()), 5);
    }
}
