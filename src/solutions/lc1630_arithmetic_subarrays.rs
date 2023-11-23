//! <https://leetcode.com/problems/arithmetic-subarrays/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        l.into_iter()
            .zip(r)
            .map(|(l, r)| {
                let mut range = nums[l as usize..=r as usize].to_vec();
                range.sort_unstable();
                range.windows(3).all(|w| w[2] - w[1] == w[1] - w[0])
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![4, 6, 5, 9, 3, 7],
                vec![0, 0, 2],
                vec![2, 3, 5]
            ),
            vec![true, false, true]
        );
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
                vec![0, 1, 6, 4, 8, 7],
                vec![4, 4, 9, 7, 9, 10]
            ),
            vec![false, true, false, false, true, true]
        );
    }
}
