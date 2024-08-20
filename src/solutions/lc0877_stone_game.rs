//! <https://leetcode.com/problems/stone-game/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn stone_game(_piles: Vec<i32>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::stone_game(vec![5, 3, 4, 5]));
        assert!(Solution::stone_game(vec![3, 7, 2, 3]));
    }
}
