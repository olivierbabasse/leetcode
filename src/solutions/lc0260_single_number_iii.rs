//! <https://leetcode.com/problems/single-number-iii/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let x = nums.iter().fold(0, |acc, num| acc ^ *num);
        let bit = x & -x;
        let mut a = 0;
        let mut b = 0;
        nums.into_iter()
            .for_each(|e| if e & bit != 0 { a ^= e } else { b ^= e });
        vec![a, b]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
        assert_eq!(Solution::single_number(vec![-1, 0]), vec![-1, 0]);
        assert_eq!(Solution::single_number(vec![0, 1]), vec![1, 0]);
    }
}
