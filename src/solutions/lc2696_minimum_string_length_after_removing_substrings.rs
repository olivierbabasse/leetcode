//! <https://leetcode.com/problems/minimum-string-length-after-removing-substrings/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if let Some(last) = stack.last() {
                if c == 'B' && *last == 'A' {
                    stack.pop();
                    continue;
                }
            }
            if let Some(last) = stack.last() {
                if c == 'D' && *last == 'C' {
                    stack.pop();
                    continue;
                }
            }
            stack.push(c);
        }
        stack.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_length("ABFCACDB".into()), 2);
        assert_eq!(Solution::min_length("ACBBD".into()), 5);
    }
}
