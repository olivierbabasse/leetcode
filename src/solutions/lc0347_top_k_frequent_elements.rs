//! <https://leetcode.com/problems/top-k-frequent-elements/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freqs: HashMap<i32, i32> = HashMap::new();
        nums.into_iter()
            .for_each(|n| *freqs.entry(n).or_default() += 1);
        let mut freqs = freqs.into_iter().map(|(n, f)| (f, n)).collect::<Vec<_>>();
        freqs.sort_unstable();
        freqs
            .into_iter()
            .rev()
            .take(k as usize)
            .map(|(_, n)| n)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
