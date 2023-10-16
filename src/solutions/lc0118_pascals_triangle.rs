//! <https://leetcode.com/problems/pascals-triangle/>

struct Solution {}

impl Solution {
    /// time-complexity : O(n^2)
    /// space-complexity : O(n^2)
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let num_rows = num_rows as usize;
        let mut vals = vec![0; num_rows];
        for i in 0..num_rows {
            vals[i] = 1;
            for j in (1..i).rev() {
                vals[j] += vals[j - 1];
            }
            res.push(vals[0..i + 1].to_vec());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
        assert_eq!(Solution::generate(2), vec![vec![1], vec![1, 1]]);
        assert_eq!(
            Solution::generate(3),
            vec![vec![1], vec![1, 1], vec![1, 2, 1]]
        );
    }
}
