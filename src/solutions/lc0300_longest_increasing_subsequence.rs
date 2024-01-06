//! <https://leetcode.com/problems/longest-increasing-subsequence/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut seq = Vec::new();
        for n in nums {
            if let Err(pos) = seq.binary_search(&n) {
                if pos == seq.len() {
                    seq.push(n);
                } else {
                    seq[pos] = n
                };
            }
        }
        seq.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
}
