//! <https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let (mut opening, mut unbalanced) = (0, 0);
        for c in s.as_bytes() {
            if *c == b'[' {
                opening += 1;
            } else if opening > 0 {
                opening -= 1;
            } else {
                unbalanced += 1;
            }
        }
        (unbalanced + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_swaps("][][".into()), 1);
        assert_eq!(Solution::min_swaps("]]][[[".into()), 2);
        assert_eq!(Solution::min_swaps("[]".into()), 0);
    }
}
