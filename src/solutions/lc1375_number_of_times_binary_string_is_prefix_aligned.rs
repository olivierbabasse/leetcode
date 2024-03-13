//! <https://leetcode.com/problems/number-of-times-binary-string-is-prefix-aligned/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let (mut count, mut max) = (0, 0);
        for (index, flip) in flips.into_iter().enumerate() {
            max = max.max(flip as usize);
            if max == index + 1 {
                count += 1
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_times_all_blue(vec![3, 2, 4, 1, 5]), 2);
        assert_eq!(Solution::num_times_all_blue(vec![4, 1, 2, 3]), 1);
    }
}
