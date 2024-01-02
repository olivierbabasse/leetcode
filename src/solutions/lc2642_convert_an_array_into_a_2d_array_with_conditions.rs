//! <https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut freqs: HashMap<i32, usize> = HashMap::new();
        let mut max = 0;
        for n in nums {
            *freqs.entry(n).or_default() += 1;
            if freqs[&n] > max {
                max = freqs[&n];
            }
        }
        let mut res = vec![Vec::new(); max];
        for (n, f) in freqs {
            for r in res.iter_mut().take(f) {
                r.push(n);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::array_of_arrays_eq;

    use super::*;

    #[test]
    fn test_cases() {
        assert!(array_of_arrays_eq(
            &Solution::find_matrix(vec![1, 3, 4, 1, 2, 3, 1]),
            &[vec![1, 3, 4, 2], vec![1, 3], vec![1]]
        ));
    }
}
