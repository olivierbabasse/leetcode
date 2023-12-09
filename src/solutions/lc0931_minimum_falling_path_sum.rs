//! <https://leetcode.com/problems/minimum-falling-path-sum/>

/// time-complexity : O(n^2)
/// space-complexity : O(n^2)
struct Solution {}

impl Solution {
    fn rec(
        line: usize,
        col: usize,
        matrix: &Vec<Vec<i32>>,
        cache: &mut Vec<Vec<Option<i32>>>,
    ) -> i32 {
        let n = matrix.len();
        if line >= n || col >= n {
            0
        } else if let Some(val) = cache[line][col] {
            val
        } else {
            let mut m = Self::rec(line + 1, col, matrix, cache);
            if col >= 1 {
                m = m.min(Self::rec(line + 1, col - 1, matrix, cache));
            }
            if col < n - 1 {
                m = m.min(Self::rec(line + 1, col + 1, matrix, cache));
            }
            let val = matrix[line][col] + m;
            cache[line][col] = Some(val);
            val
        }
    }

    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut cache = vec![vec![None; n]; n];
        (0..n)
            .map(|col| Self::rec(0, col, &matrix, &mut cache))
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]]),
            13
        );
        assert_eq!(
            Solution::min_falling_path_sum(vec![vec![-19, 57], vec![-40, -5]]),
            -59
        );
    }
}
