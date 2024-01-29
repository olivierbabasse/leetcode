//! <https://leetcode.com/problems/range-sum-query-2d-immutable/>

struct NumMatrix {
    prefix_sum: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut prefix_sum = vec![vec![0; cols + 1]; rows + 1];
        for i in 1..=rows {
            for j in 1..=cols {
                prefix_sum[i][j] =
                    prefix_sum[i - 1][j] + prefix_sum[i][j - 1] + matrix[i - 1][j - 1]
                        - prefix_sum[i - 1][j - 1];
            }
        }
        Self { prefix_sum }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.prefix_sum[row2 as usize + 1][col2 as usize + 1]
            - self.prefix_sum[row2 as usize + 1][col1 as usize]
            - self.prefix_sum[row1 as usize][col2 as usize + 1]
            + self.prefix_sum[row1 as usize][col1 as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let obj = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);
        assert_eq!(obj.sum_region(2, 1, 4, 3), 8);
        assert_eq!(obj.sum_region(1, 1, 2, 2), 11);
        assert_eq!(obj.sum_region(1, 2, 2, 4), 12);
    }
}
