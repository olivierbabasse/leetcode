//! <https://leetcode.com/problems/spiral-matrix-ii/>

struct Solution {}

/// time-complexity : O(rows*cols)
/// space-complexity : O(rows*cols)
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; n as usize]; n as usize];
        let (mut top, mut bottom, mut left, mut right, mut n) = (0i32, n - 1, 0i32, n - 1, 1);

        while top <= bottom && left <= right {
            for col in left..=right {
                res[top as usize][col as usize] = n;
                n += 1;
            }
            top += 1;

            for row in top..=bottom {
                res[row as usize][right as usize] = n;
                n += 1;
            }
            right -= 1;

            if top <= bottom {
                for col in (left..=right).rev() {
                    res[bottom as usize][col as usize] = n;
                    n += 1;
                }
                bottom -= 1;
            }

            if left <= right {
                for row in (top..=bottom).rev() {
                    res[row as usize][left as usize] = n;
                    n += 1;
                }
                left += 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }
}
