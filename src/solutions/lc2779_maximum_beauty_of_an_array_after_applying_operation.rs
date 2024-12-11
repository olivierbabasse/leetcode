//! <https://leetcode.com/problems/maximum-beauty-of-an-array-after-applying-operation/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums.iter()
            .enumerate()
            .map(|(i, n)| nums.partition_point(|&x| x <= n + 2 * k) - i)
            .max()
            .unwrap_or_default() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::maximum_beauty(vec![4, 6, 1, 2], 2), 3);
        assert_eq!(Solution::maximum_beauty(vec![1, 1, 1, 1], 10), 4);
    }
}
