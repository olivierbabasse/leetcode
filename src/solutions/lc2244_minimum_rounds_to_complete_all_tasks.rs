//! <https://leetcode.com/problems/minimum-rounds-to-complete-all-tasks/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut freqs: HashMap<i32, i32> = HashMap::new();
        tasks.into_iter().for_each(|t| {
            *freqs.entry(t).or_default() += 1;
        });
        let mut count = 0;
        for (_, f) in freqs {
            if f == 1 {
                return -1;
            } else {
                count += (f as f32 / 3.0).ceil() as i32;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]),
            4
        );
        assert_eq!(Solution::minimum_rounds(vec![2, 3, 3]), -1);
        assert_eq!(Solution::minimum_rounds(vec![7, 7, 7, 7, 7, 7]), 2);
    }
}
