//! <https://leetcode.com/problems/find-polygon-with-the-largest-perimeter/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable();
        let mut res = -1;
        let mut previous_sum = 0;
        for n in nums.into_iter() {
            let n = n as i64;
            if n < previous_sum {
                res = previous_sum + n;
            }
            previous_sum += n;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::largest_perimeter(vec![5, 5, 5]), 15);
        assert_eq!(Solution::largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]), 12);
        assert_eq!(Solution::largest_perimeter(vec![5, 5, 50]), -1);
    }
}
