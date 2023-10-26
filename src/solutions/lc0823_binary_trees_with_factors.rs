//! <https://leetcode.com/problems/binary-trees-with-factors/>

use std::collections::HashMap;

/// time-complexity : O(n^2)
/// space-complexity : O(n)
struct Solution {}

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        arr.dedup();
        arr.sort_unstable();

        let mut dp = HashMap::new();
        for i in &arr {
            dp.insert(i, 1);

            for j in &arr {
                if i % j == 0 {
                    if let Some(res) = dp.get(&(i / j)).cloned() {
                        let vj = *dp.get(j).unwrap();
                        dp.entry(i).and_modify(|v| *v += vj * res);
                    }
                }
            }
        }

        (dp.values().sum::<i64>() % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_factored_binary_trees(vec![2, 4]), 3);
        assert_eq!(Solution::num_factored_binary_trees(vec![2, 4, 5, 10]), 7);
    }
}
