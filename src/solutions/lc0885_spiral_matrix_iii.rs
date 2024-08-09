//! <https://leetcode.com/problems/spiral-matrix-iii/>

struct Solution {}

/// time-complexity : O(rows*cols)
/// space-complexity : O(rows*cols)
impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, mut r: i32, mut c: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![r, c]];
        let mut step = 1;
        let maybe_save_pos = |result: &mut Vec<Vec<i32>>, row, col| {
            if row >= 0 && col >= 0 && row < rows && col < cols {
                result.push(vec![row, col]);
            }
        };
        while result.len() < (rows * cols) as usize {
            for _ in 0..step {
                if step % 2 == 0 {
                    c -= 1;
                    maybe_save_pos(&mut result, r, c);
                } else {
                    c += 1;
                    maybe_save_pos(&mut result, r, c);
                }
            }
            for _ in 0..step {
                if step % 2 == 0 {
                    r -= 1;
                    maybe_save_pos(&mut result, r, c);
                } else {
                    r += 1;
                    maybe_save_pos(&mut result, r, c);
                }
            }
            step += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::spiral_matrix_iii(1, 4, 0, 0),
            vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]
        );
    }
}
