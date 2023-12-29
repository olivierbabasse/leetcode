//! <https://leetcode.com/problems/earliest-possible-day-of-full-bloom/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut grow_and_plant_time = grow_time.into_iter().zip(plant_time).collect::<Vec<_>>();
        grow_and_plant_time.sort_unstable();
        let mut pt = 0;
        let mut max = 0;
        for gpt in grow_and_plant_time.into_iter().rev() {
            pt += gpt.1;
            max = max.max(pt + gpt.0);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::earliest_full_bloom(vec![1, 4, 3], vec![2, 3, 1]),
            9
        );
        assert_eq!(
            Solution::earliest_full_bloom(vec![1, 2, 3, 2], vec![2, 1, 2, 1]),
            9
        );
        assert_eq!(Solution::earliest_full_bloom(vec![1], vec![1]), 2);
    }
}
