//! <https://leetcode.com/problems/largest-substring-between-two-equal-characters/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut start_pos = vec![None; 26];
        let mut max = -1;
        for (i, c) in s.bytes().enumerate() {
            max = max.max(i as i32 - *start_pos[(c - b'a') as usize].get_or_insert(i as i32) - 1);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_length_between_equal_characters("aa".into()),
            0
        );
        assert_eq!(
            Solution::max_length_between_equal_characters("abca".into()),
            2
        );
        assert_eq!(
            Solution::max_length_between_equal_characters("cbzxy".into()),
            -1
        );
    }
}
