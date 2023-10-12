//! <https://leetcode.com/problems/find-in-mountain-array/>

struct Solution {}

impl Solution {
    /// time-complexity : O(log(n))
    /// space-complexity : O(1)
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut begin = 0;
        let mut end = arr.len() - 1;
        while begin < end {
            let middle = (begin + end) / 2;
            if arr[middle] > arr[middle + 1] {
                end = middle;
            } else {
                begin = middle + 1;
            }
        }
        begin as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
        assert_eq!(
            Solution::peak_index_in_mountain_array(vec![0, 3, 5, 12, 2]),
            3
        );
    }
}
