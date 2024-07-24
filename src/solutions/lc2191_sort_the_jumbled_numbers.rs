//! <https://leetcode.com/problems/sort-the-jumbled-numbers/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut res = nums
            .iter()
            .enumerate()
            .map(|(index, num)| {
                (
                    format!("{num}")
                        .bytes()
                        .fold(0, |acc, b| acc * 10 + mapping[(b - b'0') as usize]),
                    index,
                )
            })
            .collect::<Vec<_>>();
        res.sort_unstable();
        res.into_iter().map(|(_, index)| nums[index]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38]),
            vec![338, 38, 991]
        );
        assert_eq!(
            Solution::sort_jumbled(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![789, 456, 123]),
            vec![123, 456, 789]
        );
    }
}
