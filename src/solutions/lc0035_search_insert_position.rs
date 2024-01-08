//! <https://leetcode.com/problems/search-insert-position/>

use std::cmp::Ordering;

struct Solution {}

/// time-complexity : O(log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut begin, mut end) = (0i32, nums.len() as i32 - 1);
        while begin <= end {
            let mid = (begin + end) / 2;
            match target.cmp(&nums[mid as usize]) {
                Ordering::Less => {
                    end = mid - 1;
                }
                Ordering::Greater => {
                    begin = mid + 1;
                }
                Ordering::Equal => {
                    return mid;
                }
            }
        }
        begin
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
        assert_eq!(Solution::search_insert(vec![1, 3], 2), 1);
    }
}
