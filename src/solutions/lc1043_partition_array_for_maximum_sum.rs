//! <https://leetcode.com/problems/partition-array-for-maximum-sum/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn rec(arr: &[i32], start: usize, k: i32, cache: &mut Vec<Option<i32>>) -> i32 {
        if start >= arr.len() {
            0
        } else if let Some(res) = cache[start] {
            res
        } else {
            let mut curmax = 0;
            let mut res = 0;
            let end = usize::min(arr.len(), start + k as usize);
            for i in start..end {
                curmax = curmax.max(arr[i]);
                res = res.max(curmax * (i - start + 1) as i32 + Self::rec(arr, i + 1, k, cache));
            }
            cache[start] = Some(res);
            res
        }
    }

    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let mut cache = vec![None; arr.len()];
        Self::rec(&arr, 0, k, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3),
            84
        );
        assert_eq!(
            Solution::max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4),
            83
        );
        assert_eq!(Solution::max_sum_after_partitioning(vec![1], 1), 1);
    }
}
