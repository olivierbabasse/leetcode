//! <https://leetcode.com/problems/pass-the-pillow/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let trips = time / (n - 1);
        let left = time % (n - 1);
        if trips % 2 == 0 {
            left + 1
        } else {
            n - left
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::pass_the_pillow(4, 5), 2);
        assert_eq!(Solution::pass_the_pillow(3, 2), 3);
    }
}
