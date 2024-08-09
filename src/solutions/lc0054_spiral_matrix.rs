//! <https://leetcode.com/problems/spiral-matrix/>

struct Solution {}

/// time-complexity : O(rows*cols)
/// space-complexity : O(rows*cols)
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut top, mut bottom, mut left, mut right) = (
            0i32,
            matrix.len() as i32 - 1,
            0i32,
            matrix[0].len() as i32 - 1,
        );
        let mut res = Vec::new();

        while top <= bottom && left <= right {
            for col in left..=right {
                res.push(matrix[top as usize][col as usize]);
            }
            top += 1;

            for row in top..=bottom {
                res.push(matrix[row as usize][right as usize]);
            }
            right -= 1;

            if top <= bottom {
                for col in (left..=right).rev() {
                    res.push(matrix[bottom as usize][col as usize]);
                }
                bottom -= 1;
            }

            if left <= right {
                for row in (top..=bottom).rev() {
                    res.push(matrix[row as usize][left as usize]);
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
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
        assert_eq!(Solution::spiral_order(vec![vec![3], vec![2]]), vec![3, 2]);
    }
}
