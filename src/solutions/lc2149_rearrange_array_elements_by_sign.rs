//! <https://leetcode.com/problems/rearrange-array-elements-by-sign/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let (pos, neg): (Vec<_>, Vec<_>) = nums.into_iter().partition(|&n| n >= 0);

        pos.into_iter()
            .zip(neg)
            .flat_map(|(p, n)| vec![p, n])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4]),
            vec![3, -2, 1, -5, 2, -4]
        );
        assert_eq!(Solution::rearrange_array(vec![-1, 1]), vec![1, -1]);
    }
}
