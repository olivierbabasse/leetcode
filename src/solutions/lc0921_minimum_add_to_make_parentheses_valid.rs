//! <https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let (mut open, mut add) = (0, 0);
        for &b in s.as_bytes() {
            if b == b'(' {
                open += 1;
            } else if open > 0 {
                open -= 1;
            } else {
                add += 1;
            }
        }
        add + open
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_add_to_make_valid("())".into()), 1);
        assert_eq!(Solution::min_add_to_make_valid("(((".into()), 3);
    }
}
