//! <https://leetcode.com/problems/find-target-indices-after-sorting-array/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort_unstable();
        nums.into_iter()
            .enumerate()
            .filter(|(_, num)| num == &target)
            .map(|(index, _)| index as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 2), vec![1, 2]);
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 3), vec![3]);
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 5), vec![4]);
    }
}
