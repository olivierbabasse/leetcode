//! <https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let dx = (fx - sx).abs();
        let dy = (fy - sy).abs();
        let min = dx.max(dy);

        if t >= 2 {
            t >= min
        } else {
            t == min
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::is_reachable_at_time(2, 4, 7, 7, 6));
        assert!(!Solution::is_reachable_at_time(3, 1, 7, 3, 3));
        assert!(Solution::is_reachable_at_time(1, 4, 1, 3, 1));
    }
}
