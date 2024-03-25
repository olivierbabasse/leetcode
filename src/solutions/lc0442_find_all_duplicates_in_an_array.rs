//! <https://leetcode.com/problems/find-all-duplicates-in-an-array/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();

        for i in 0..nums.len() {
            let pos = (nums[i].abs() - 1) as usize;
            if nums[pos] < 0 {
                res.push(nums[i].abs());
            } else {
                nums[pos] = -nums[pos];
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );
        assert_eq!(Solution::find_duplicates(vec![1, 1, 2]), vec![1]);
    }
}
