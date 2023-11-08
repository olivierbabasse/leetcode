//! <https://leetcode.com/problems/reaching-points/>

use std::cmp::Ordering;

struct Solution {}

/// time-complexity : O(log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, mut x: i32, mut y: i32) -> bool {
        loop {
            if x == sx && y == sy {
                return true;
            }
            if x < sx || y < sy {
                return false;
            }
            if x == sx {
                return (y - sy) % x == 0;
            }
            if y == sy {
                return (x - sx) % y == 0;
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
        assert!(Solution::reaching_points(1, 1, 3, 5));
        assert!(!Solution::reaching_points(1, 1, 2, 2));
        assert!(Solution::reaching_points(1, 1, 1, 1));
        assert!(!Solution::reaching_points(3, 7, 3, 4));
    }
}
