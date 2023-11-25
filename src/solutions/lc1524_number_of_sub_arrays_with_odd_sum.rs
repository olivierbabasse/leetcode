//! <https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let (mut odd, mut even, mut sum, mut total) = (0, 1, 0, 0i64);
        arr.into_iter().for_each(|num| {
            sum += num;
            if sum % 2 == 0 {
                even += 1;
                total += odd;
            } else {
                odd += 1;
                total += even;
            }
        });
        (total % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_of_subarrays(vec![1, 3, 5]), 4);
        assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0);
    }
}
