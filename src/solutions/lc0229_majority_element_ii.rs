//! <https://leetcode.com/problems/majority-element-ii/>

struct Solution {}

/// extending Boyer-Moore majority voting algorithm to two candidates
/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut votes1 = 0;
        let mut candidate1 = None;
        let mut votes2 = 0;
        let mut candidate2 = None;

        for num in &nums {
            if votes1 == 0 && (candidate2.is_none() || *num != candidate2.unwrap()) {
                candidate1 = Some(*num);
                votes1 = 1;
            } else if votes2 == 0 && (candidate1.is_none() || *num != candidate1.unwrap()) {
                candidate2 = Some(*num);
                votes2 = 1;
            } else if candidate1.is_some() && *num == candidate1.unwrap() {
                votes1 += 1;
            } else if candidate2.is_some() && *num == candidate2.unwrap() {
                votes2 += 1;
            } else {
                votes1 -= 1;
                votes2 -= 1;
            }
        }

        let mut results = Vec::new();
        if let Some(candidate1) = candidate1 {
            if nums.iter().filter(|n| n == &&candidate1).count() > nums.len() / 3 {
                results.push(candidate1);
            }
        }
        if let Some(candidate2) = candidate2 {
            if nums.iter().filter(|n| n == &&candidate2).count() > nums.len() / 3 {
                results.push(candidate2);
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::utils::array_eq;

    #[test]
    fn test_cases() {
        assert!(array_eq(&Solution::majority_element(vec![3, 2, 3]), &[3]));
        assert!(array_eq(&Solution::majority_element(vec![1]), &[1]));
        assert!(array_eq(&Solution::majority_element(vec![1, 2]), &[1, 2]));
        assert!(array_eq(&Solution::majority_element(vec![2, 2]), &[2]));
        assert!(array_eq(
            &Solution::majority_element(vec![-1, -1, -1]),
            &[-1]
        ));
        assert!(array_eq(
            &Solution::majority_element(vec![1, 1, 1, 3, 3, 2, 2, 2]),
            &[1, 2]
        ));
    }
}
