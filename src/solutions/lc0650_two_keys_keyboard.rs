//! <https://leetcode.com/problems/2-keys-keyboard/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn min_steps(mut n: i32) -> i32 {
        let (mut res, mut div) = (0, 2);
        while n > 1 {
            while n % div == 0 {
                res += div;
                n /= div;
            }
            div += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_steps(3), 3);
        assert_eq!(Solution::min_steps(1), 0);
    }
}
