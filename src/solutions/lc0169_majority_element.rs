//! <https://leetcode.com/problems/majority-element/>

struct Solution {}

/// using Boyer-Moore majority voting algorithm
/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut votes = 0;
        let mut candidate = -1;

        for num in nums {
            if votes == 0 {
                candidate = num;
                votes = 1;
            } else if num == candidate {
                votes += 1;
            } else {
                votes -= 1;
            }
        }

        // not doing check pass since we known there is a majority element

        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
