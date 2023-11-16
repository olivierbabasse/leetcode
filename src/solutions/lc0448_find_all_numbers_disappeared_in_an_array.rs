//! <https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let index = (nums[i].abs() - 1) as usize;
            nums[index] = -nums[index].abs()
        }

        nums.into_iter()
            .enumerate()
            .flat_map(|(index, n)| if n > 0 { Some(index as i32 + 1) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
        assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
    }
}
