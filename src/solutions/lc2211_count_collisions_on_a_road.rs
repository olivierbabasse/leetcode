//! <https://leetcode.com/problems/count-collisions-on-a-road/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        directions
            .trim_start_matches('L')
            .trim_end_matches('R')
            .chars()
            .filter(|&c| c != 'S')
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_collisions("RLRSLL".into()), 5);
        assert_eq!(Solution::count_collisions("LLRR".into()), 0);
    }
}
