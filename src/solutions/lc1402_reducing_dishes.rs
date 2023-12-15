//! <https://leetcode.com/problems/reducing-dishes/description/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    fn dp(
        ndish: usize,
        index: usize,
        satisfaction: &Vec<i32>,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if index >= satisfaction.len() {
            0
        } else if let Some(val) = cache.get(&(ndish, index)) {
            *val
        } else {
            let val = i32::max(
                satisfaction[index] * ndish as i32
                    + Self::dp(ndish + 1, index + 1, satisfaction, cache),
                Self::dp(ndish, index + 1, satisfaction, cache),
            );
            cache.insert((ndish, index), val);
            val
        }
    }

    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        satisfaction.sort_unstable();
        Self::dp(1, 0, &satisfaction, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]), 14);
        assert_eq!(Solution::max_satisfaction(vec![4, 3, 2]), 20);
        assert_eq!(Solution::max_satisfaction(vec![-1, -4, -5]), 0);
    }
}
