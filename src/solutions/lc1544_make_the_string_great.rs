//! <https://leetcode.com/problems/make-the-string-great/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack = Vec::<char>::new();
        for cur in s.chars() {
            match stack.last() {
                Some(&prev)
                    if cur.to_ascii_lowercase() == prev.to_ascii_lowercase()
                        && (cur.is_lowercase() ^ prev.is_lowercase()) =>
                {
                    stack.pop();
                }
                _ => {
                    stack.push(cur);
                }
            }
        }
        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::make_good("leEeetcode".into()),
            "leetcode".to_string()
        );
        assert_eq!(Solution::make_good("abBAcC".into()), "".to_string());
        assert_eq!(Solution::make_good("s".into()), "s".to_string());
    }
}
