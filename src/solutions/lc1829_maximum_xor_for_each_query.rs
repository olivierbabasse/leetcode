//! <https://leetcode.com/problems/maximum-xor-for-each-query/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut xor = nums.iter().fold(0, |acc, x| acc ^ x);
        let mut res = vec![0; nums.len()];
        let mask = (1 << maximum_bit) - 1;
        for (i, n) in nums.into_iter().rev().enumerate() {
            res[i] = xor ^ mask;
            xor ^= n;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::get_maximum_xor(vec![0, 1, 1, 3], 2),
            vec![0, 3, 2, 3]
        );
        assert_eq!(
            Solution::get_maximum_xor(vec![2, 3, 4, 7], 3),
            vec![5, 2, 6, 5]
        );
        assert_eq!(
            Solution::get_maximum_xor(vec![0, 1, 2, 2, 5, 7], 3),
            vec![4, 3, 6, 4, 6, 7]
        );
    }
}
