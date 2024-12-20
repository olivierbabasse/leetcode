//! <https://leetcode.com/problems/max-chunks-to-make-sorted/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut prefix_max = arr.clone();
        let mut suffix_min = arr.clone();

        for i in 1..n {
            prefix_max[i] = prefix_max[i].max(prefix_max[i - 1]);
        }

        for i in (0..n - 1).rev() {
            suffix_min[i] = suffix_min[i].min(suffix_min[i + 1]);
        }

        1 + (1..n)
            .filter(|&i| suffix_min[i] > prefix_max[i - 1])
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
        assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
    }
}
