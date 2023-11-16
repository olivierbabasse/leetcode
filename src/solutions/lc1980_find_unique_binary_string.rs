//! <https://leetcode.com/problems/find-unique-binary-string/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        nums.into_iter()
            .enumerate()
            .map(|(i, s)| if s.as_bytes()[i] == b'0' { '1' } else { '0' })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_different_binary_string(vec!["01".into(), "10".into()]),
            "11".to_string()
        );
        assert_eq!(
            Solution::find_different_binary_string(vec!["00".into(), "01".into()]),
            "10".to_string()
        );
        assert_eq!(
            Solution::find_different_binary_string(vec!["111".into(), "011".into(), "001".into()]),
            "000".to_string()
        );
    }
}
