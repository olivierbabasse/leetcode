//! <https://leetcode.com/problems/count-subarrays-with-score-less-than-k/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let (mut res, mut cur) = (0, 0i64);
        let (mut begin, mut end) = (0, 0);

        while end < nums.len() {
            cur += nums[end] as i64;

            while cur * (end - begin + 1) as i64 >= k {
                cur -= nums[begin] as i64;
                begin += 1;
            }

            res += end - begin + 1;

            end += 1;
        }

        res as i64
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_subarrays(vec![2, 1, 4, 3, 5], 10), 6);
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1], 5), 5);
    }
}
