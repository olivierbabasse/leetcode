//! <https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(n)
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut cur = -1;
        let mut cur_count = 0;
        for &i in arr.iter() {
            if i != cur {
                cur = i;
                cur_count = 0;
            }
            cur_count += 1;
            if cur_count as f64 > (arr.len() as f64 / 4.0) {
                return cur;
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
        assert_eq!(
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
            6
        );
        assert_eq!(Solution::find_special_integer(vec![1, 1]), 1);
        assert_eq!(Solution::find_special_integer(vec![1]), 1);
        assert_eq!(Solution::find_special_integer(vec![1, 2, 3, 3]), 3);
    }
}
