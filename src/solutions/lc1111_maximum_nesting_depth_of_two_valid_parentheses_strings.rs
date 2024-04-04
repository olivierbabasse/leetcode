//! <https://leetcode.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut depth = 0;
        let mut output = Vec::new();
        for c in seq.chars() {
            if c == '(' {
                depth += 1;
            }
            output.push(depth % 2);
            if c == ')' {
                depth -= 1;
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_depth_after_split("(()())".into()),
            vec![1, 0, 0, 0, 0, 1]
        );
        assert_eq!(
            Solution::max_depth_after_split("()(())()".into()),
            vec![1, 1, 1, 0, 0, 1, 1, 1]
        );
    }
}
