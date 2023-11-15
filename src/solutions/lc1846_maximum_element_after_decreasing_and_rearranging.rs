//! <https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        let mut last = 0;
        for a in arr {
            if (a - last).abs() > 1 {
                last += 1;
            } else {
                last = a
            }
        }
        last
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1]),
            2
        );
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![100, 1, 1000]),
            3
        );
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 2, 3, 4, 5]),
            5
        );
    }
}
