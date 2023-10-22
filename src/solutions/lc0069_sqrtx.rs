//! <https://leetcode.com/problems/sqrtx/>

use std::cmp::Ordering;

struct Solution {}

/// time-complexity : O(log(x))
/// space-complexity : O(1)
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut low = 0;
        let mut high = 65535;
        while low <= high {
            let mid = low + (high - low) / 2;
            match (x as i64).cmp(&((mid as i64) * (mid as i64))) {
                Ordering::Equal => return mid,
                Ordering::Greater => low = mid + 1,
                Ordering::Less => high = mid - 1,
            }
        }
        high
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::my_sqrt(2), 1);
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(9), 3);
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }
}
