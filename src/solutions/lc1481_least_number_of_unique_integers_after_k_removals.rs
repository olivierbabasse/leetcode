//! <https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut freqs: HashMap<i32, usize> = HashMap::new();
        arr.into_iter()
            .for_each(|n| *freqs.entry(n).or_default() += 1);
        let mut freq_nums = freqs.into_iter().map(|(n, f)| (f, n)).collect::<Vec<_>>();
        freq_nums.sort_unstable();
        for (i, (f, _)) in freq_nums.iter().enumerate() {
            k -= *f as i32;
            if k <= 0 {
                return (freq_nums.len() - i - if k == 0 { 1 } else { 0 }) as i32;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_least_num_of_unique_ints(vec![5, 5, 4], 1), 1);
        assert_eq!(
            Solution::find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3),
            2
        );
    }
}
