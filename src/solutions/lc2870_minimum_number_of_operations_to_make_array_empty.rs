//! <https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    fn rec(nums: &[i32], index: usize, cache: &mut Vec<Option<i32>>) -> i32 {
        let n = nums.len();
        if index >= n {
            return 0;
        }
        if let Some(res) = cache[index] {
            return res;
        }

        let (mut a, mut b) = (-1, -1);
        if n >= index + 2 && nums[index] == nums[index + 1] {
            let r = Self::rec(nums, index + 2, cache);
            if r >= 0 {
                a = 1 + r;
            }
        };
        if n >= index + 3 && nums[index] == nums[index + 1] && nums[index + 1] == nums[index + 2] {
            let r = Self::rec(nums, index + 3, cache);
            if r >= 0 {
                b = 1 + r;
            }
        };

        let res = if a == -1 && b == -1 {
            -1
        } else if a == -1 {
            b
        } else if b == -1 {
            a
        } else {
            a.min(b)
        };
        cache[index] = Some(res);
        res
    }

    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut cache = vec![None; nums.len() + 1];
        nums.sort_unstable();
        Self::rec(&nums, 0, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_operations(vec![2, 3, 3, 2, 2, 4, 2, 3, 4]), 4);
        assert_eq!(Solution::min_operations(vec![2, 1, 2, 2, 3, 3]), -1);
    }
}
