//! <https://leetcode.com/problems/bulb-switcher/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::bulb_switch(3), 1);
        assert_eq!(Solution::bulb_switch(0), 0);
        assert_eq!(Solution::bulb_switch(1), 1);
    }
}
