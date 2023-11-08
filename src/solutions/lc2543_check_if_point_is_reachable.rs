//! <https://leetcode.com/problems/check-if-point-is-reachable/>

use std::cmp::Ordering;

struct Solution {}

/// time-complexity : O(log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn is_reachable(mut x: i32, mut y: i32) -> bool {
        loop {
            while x % 2 == 0 {
                x /= 2;
            }
            while y % 2 == 0 {
                y /= 2;
            }
            if x == 1 && y == 1 {
                return true;
            }
            match x.cmp(&y) {
                Ordering::Equal => return false,
                Ordering::Greater => x -= y,
                Ordering::Less => y -= x,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(!Solution::is_reachable(6, 9));
        assert!(Solution::is_reachable(4, 7));
    }
}
