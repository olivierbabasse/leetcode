//! <https://leetcode.com/problems/longest-common-prefix/>

struct Solution {}

/// time-complexity : O(n*len(shortest))
/// space-complexity : O(len(shortest))
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();
        for (pos, c) in strs[0].chars().enumerate() {
            if strs.iter().all(|s| s.chars().nth(pos) == Some(c)) {
                prefix.push(c);
            } else {
                break;
            }
        }
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".into()]),
            "".to_string()
        );
    }
}
