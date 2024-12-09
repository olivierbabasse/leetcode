//! <https://leetcode.com/problems/special-array-ii/>

struct Solution {}

/// time-complexity : O(len(q)*log(len(nums)))
/// space-complexity : O(len(nums))
impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut bad = Vec::new();
        for i in 1..nums.len() {
            if nums[i] % 2 == nums[i - 1] % 2 {
                bad.push(i as i32);
            }
        }

        queries
            .into_iter()
            .map(|q| {
                let a = bad.partition_point(|&x| x < q[0] + 1);
                let b = bad.partition_point(|&x| x <= q[1]);
                a == b
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
            Solution::is_array_special(vec![3, 4, 1, 2, 6], vec![vec![0, 4]]),
            vec![false]
        );
        assert_eq!(
            Solution::is_array_special(vec![4, 3, 1, 6], vec![vec![0, 2], vec![2, 3]]),
            vec![false, true]
        );
    }
}
