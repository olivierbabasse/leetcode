//! <https://leetcode.com/problems/sum-of-distances/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut res = vec![0; nums.len()];
        let mut numposps: HashMap<i32, Vec<_>> = HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            numposps.entry(n).or_default().push(i + 1);
        }
        for v in numposps.values_mut() {
            for i in 1..v.len() {
                v[i] += v[i - 1];
            }
        }

        let mut indexfreq: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            if numposps.get(&nums[i]).unwrap().len() == 1 {
                continue;
            }

            *indexfreq.entry(nums[i]).or_default() += 1;
            let pos = *(indexfreq.get(&nums[i]).unwrap()) as usize;

            let v = numposps.get(&nums[i]).unwrap();
            if pos == 1 {
                res[i] = v.last().unwrap() - v[0] * v.len();
                continue;
            }

            let prefix_sum = v.last().unwrap() - v[pos - 2];
            let postfix_sum = v[pos - 2];

            res[i] = (prefix_sum - ((v.len() - pos + 1) * (i + 1))) + ((pos - 1) * (i + 1))
                - postfix_sum;
        }

        res.into_iter().map(|x| x as i64).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::distance(vec![1, 3, 1, 1, 2]), vec![5, 0, 3, 4, 0]);
        assert_eq!(Solution::distance(vec![0, 5, 3]), vec![0, 0, 0]);
    }
}
