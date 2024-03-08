//! <https://leetcode.com/problems/count-elements-with-maximum-frequency/>

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut freqs = vec![0; 101];
        for &n in &nums {
            freqs[n as usize] += 1;
        }
        let max = *freqs.iter().max().unwrap_or(&0);
        freqs.into_iter().filter(|&f| f == max).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
    }
}
