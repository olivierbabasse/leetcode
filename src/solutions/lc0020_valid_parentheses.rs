//! <https://leetcode.com/problems/valid-parentheses/>

struct Solution {}

impl Solution {
    /// time-complexity : O(n)
    /// space-complexity : O(1)
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                _ => match stack.pop() {
                    Some('(') if c == ')' => {}
                    Some('[') if c == ']' => {}
                    Some('{') if c == '}' => {}
                    _ => return false,
                },
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert!(Solution::is_valid("()".into()));
        assert!(Solution::is_valid("()[]{}".into()));
        assert!(!Solution::is_valid("(]".into()));
    }
}
