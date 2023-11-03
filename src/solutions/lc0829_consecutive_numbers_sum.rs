//! <https://leetcode.com/problems/consecutive-numbers-sum/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let n = n as i64;
        let mut count = 1;

        for k in 2..n {
            if k * (k - 1) >= 2 * n {
                break;
            }
            if (n - k * (k - 1) / 2) % k == 0 {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::consecutive_numbers_sum(5), 2);
        assert_eq!(Solution::consecutive_numbers_sum(9), 3);
        assert_eq!(Solution::consecutive_numbers_sum(15), 4);
        assert_eq!(Solution::consecutive_numbers_sum(93003), 8);
    }
}
