//! <https://leetcode.com/problems/k-th-symbol-in-grammar/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 0 {
            return 0;
        }

        let mid = 2i32.pow(n as u32) / 2;

        if k <= mid {
            Self::kth_grammar(n - 1, k)
        } else {
            1 - Self::kth_grammar(n - 1, k - mid)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
    }
}
