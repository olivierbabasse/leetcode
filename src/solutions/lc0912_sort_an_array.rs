//! <https://leetcode.com/problems/sort-an-array/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    fn merge_sort(nums: &mut [i32], temp: &mut [i32]) {
        let len = nums.len();
        if len <= 1 {
            return;
        }

        let mid = len / 2;
        Self::merge_sort(&mut nums[0..mid], temp);
        Self::merge_sort(&mut nums[mid..], temp);

        let (mut i, mut j, mut k) = (0, mid, 0);
        while i < mid && j < len {
            if nums[i] < nums[j] {
                temp[k] = nums[i];
                i += 1;
            } else {
                temp[k] = nums[j];
                j += 1;
            }
            k += 1;
        }
        while i < mid {
            temp[k] = nums[i];
            i += 1;
            k += 1;
        }
        while j < len {
            temp[k] = nums[j];
            j += 1;
            k += 1;
        }

        nums.copy_from_slice(&temp[0..len]);
    }

    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut temp = vec![0; nums.len()];
        Self::merge_sort(&mut nums, &mut temp);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
        assert_eq!(
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
            vec![0, 0, 1, 1, 2, 5]
        );
    }
}
