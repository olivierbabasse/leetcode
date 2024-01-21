//! <https://leetcode.com/problems/sum-of-subarray-minimums/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let mut left = Vec::new();
        let mut right = Vec::new();

        let mut stack = Vec::<i32>::new();
        for i in 0..len {
            while !stack.is_empty() && arr[*stack.last().unwrap() as usize] >= arr[i] {
                stack.pop();
            }
            left.push(*stack.last().unwrap_or(&-1));
            stack.push(i as i32);
        }

        let mut stack = Vec::<i32>::new();
        for i in (0..len).rev() {
            while !stack.is_empty() && arr[*stack.last().unwrap() as usize] > arr[i] {
                stack.pop();
            }
            right.push(*stack.last().unwrap_or(&(len as i32)));
            stack.push(i as i32);
        }
        right.reverse();

        let mut sum = 0i64;
        for i in 0..len {
            sum += (i as i64 - left[i] as i64) * (right[i] as i64 - i as i64) * arr[i] as i64;
            sum %= 1000000007;
        }
        sum as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
        assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }
}
