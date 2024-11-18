//! <https://leetcode.com/problems/shortest-subarray-to-be-removed-to-make-array-sorted/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut right = arr.len() - 1;
        while right > 0 && arr[right] >= arr[right - 1] {
            right -= 1;
        }

        let mut res = right;
        let mut left = 0;
        while left < right && (left == 0 || arr[left - 1] <= arr[left]) {
            while right < arr.len() && arr[left] > arr[right] {
                right += 1;
            }
            res = res.min(right - left - 1);
            left += 1;
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]),
            3
        );
        assert_eq!(
            Solution::find_length_of_shortest_subarray(vec![5, 4, 3, 2, 1]),
            4
        );
        assert_eq!(Solution::find_length_of_shortest_subarray(vec![1, 2, 3]), 0);
    }
}
