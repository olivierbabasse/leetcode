//! <https://leetcode.com/problems/find-the-winner-of-the-circular-game/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut res = 0;
        for i in 2..=n {
            res = (res + k) % i;
        }
        res + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_the_winner(5, 2), 3);
        assert_eq!(Solution::find_the_winner(6, 5), 1);
    }
}
