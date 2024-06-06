//! <https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/>

use std::collections::BTreeMap;

struct Solution {}

/// time-complexity : O(n * log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() % k as usize != 0 {
            return false;
        }

        let mut freqs = BTreeMap::<i32, i32>::new();
        nums.into_iter()
            .for_each(|v| *freqs.entry(v).or_default() += 1);

        while !freqs.is_empty() {
            let cur = *freqs.first_key_value().unwrap().0;
            for i in 0..k {
                if !freqs.contains_key(&(cur + i)) {
                    return false;
                }
                *freqs.get_mut(&(cur + i)).unwrap() -= 1;
                if *freqs.get(&(cur + i)).unwrap() == 0 {
                    freqs.remove(&(cur + i));
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::is_possible_divide(
            vec![1, 2, 3, 3, 4, 4, 5, 6],
            4
        ));
        assert!(Solution::is_possible_divide(
            vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11],
            3
        ));
        assert!(!Solution::is_possible_divide(vec![1, 2, 3, 4], 3));
    }
}
