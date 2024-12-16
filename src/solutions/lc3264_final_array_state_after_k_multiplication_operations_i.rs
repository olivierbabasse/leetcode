//! <https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-i/>

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}

/// time-complexity : O(k*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut nums: BinaryHeap<(Reverse<i32>, Reverse<usize>)> = nums
            .into_iter()
            .enumerate()
            .map(|(index, n)| (Reverse(n), Reverse(index)))
            .collect();
        let mut res = vec![0; nums.len()];
        for _ in 0..k {
            let (Reverse(n), rindex) = nums.pop().unwrap();
            nums.push((Reverse(n * multiplier), rindex));
        }
        for (Reverse(n), Reverse(index)) in nums {
            res[index] = n;
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
            Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2),
            vec![8, 4, 6, 5, 6]
        );
        assert_eq!(Solution::get_final_state(vec![1, 2], 3, 4), vec![16, 8]);
    }
}
