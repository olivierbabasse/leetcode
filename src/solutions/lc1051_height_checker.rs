//! <https://leetcode.com/problems/height-checker/>

struct Solution {}

/// time-complexity : O(n.log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut heights_sorted = heights.clone();
        heights_sorted.sort_unstable();
        heights
            .into_iter()
            .zip(heights_sorted)
            .filter(|(a, b)| a != b)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
        assert_eq!(Solution::height_checker(vec![5, 1, 2, 3, 4]), 5);
        assert_eq!(Solution::height_checker(vec![1, 2, 3, 4, 5]), 0);
    }
}
