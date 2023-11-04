//! <https://leetcode.com/problems/movement-of-robots/>

struct Solution {}

/// time-complexity : O(n*(log(n)))
/// space-complexity : O(1)
impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        let mut total = 0;
        let mut nums = nums.into_iter().map(|val| val as i64).collect::<Vec<_>>();

        for (i, num) in nums.iter_mut().enumerate() {
            *num += (if s.as_bytes()[i] == b'R' { d } else { -d }) as i64;
        }

        nums.sort_unstable();
        let mut prefix = 0;
        for (i, num) in nums.into_iter().enumerate() {
            total = (total + ((i as i64) * num - prefix)) % 1000000007;
            prefix += num;
        }

        total as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::sum_distance(vec![-2, 0, 2], "RLL".into(), 3), 8);
        assert_eq!(Solution::sum_distance(vec![1, 0], "RL".into(), 2), 5);
        assert_eq!(
            Solution::sum_distance(vec![2000000000, -2000000000], "RL".into(), 1000000000),
            999999965
        );
    }
}
