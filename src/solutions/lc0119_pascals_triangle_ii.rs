//! <https://leetcode.com/problems/pascals-triangle-ii/>

struct Solution {}

impl Solution {
    /// time-complexity : O(n^2)
    /// space-complexity : O(n)
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut vals = vec![0; row_index + 1];
        for i in 0..=row_index {
            vals[i] = 1;
            for j in (1..i).rev() {
                vals[j] += vals[j - 1];
            }
        }
        vals
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }
}
