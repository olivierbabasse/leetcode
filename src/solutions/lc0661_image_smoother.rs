//! <https://leetcode.com/problems/image-smoother/>

struct Solution {}

/// time-complexity : O(n*m)
/// space-complexity : O(n*m)
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (img.len(), img[0].len());
        let mut res = vec![vec![0; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                let (mut total, mut count) = (0, 0);
                let (i, j) = (i as i32, j as i32);
                for ii in -1..=1 {
                    for jj in -1..=1 {
                        if i + ii >= 0
                            && i + ii < rows as i32
                            && j + jj >= 0
                            && j + jj < cols as i32
                        {
                            total += img[(i + ii) as usize][(j + jj) as usize];
                            count += 1;
                        }
                    }
                }
                res[i as usize][j as usize] = total / count;
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
            Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        )
    }
}
