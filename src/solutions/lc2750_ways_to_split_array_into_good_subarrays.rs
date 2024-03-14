//! <https://leetcode.com/problems/ways-to-split-array-into-good-subarrays/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn number_of_good_subarray_splits(nums: Vec<i32>) -> i32 {
        let ones = nums
            .into_iter()
            .enumerate()
            .filter_map(|(i, n)| if n == 1 { Some(i) } else { None })
            .collect::<Vec<_>>();

        if ones.is_empty() {
            0
        } else {
            ones.windows(2)
                .fold(1i64, |res, w| (res * (w[1] - w[0]) as i64) % 1000000007) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::number_of_good_subarray_splits(vec![0, 0, 0]), 0);
        assert_eq!(
            Solution::number_of_good_subarray_splits(vec![0, 1, 0, 0, 1]),
            3
        );
        assert_eq!(Solution::number_of_good_subarray_splits(vec![0, 1, 0]), 1);
    }
}
