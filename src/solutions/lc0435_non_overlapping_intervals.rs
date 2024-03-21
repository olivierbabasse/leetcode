//! <https://leetcode.com/problems/non-overlapping-intervals/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut count = 0;
        let mut last = intervals[0][1];
        for i in intervals.into_iter().skip(1) {
            if i[0] >= last {
                last = i[1];
            } else {
                count += 1;
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
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
            2
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
            0
        );
    }
}
