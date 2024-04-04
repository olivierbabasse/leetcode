//! <https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut depth = 0;
        let mut max = 0;
        for c in s.chars() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            }
            max = max.max(depth);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".into()), 3);
        assert_eq!(Solution::max_depth("(1)+((2))+(((3)))".into()), 3);
    }
}
