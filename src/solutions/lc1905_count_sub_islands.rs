//! <https://leetcode.com/problems/count-sub-islands/description/>

struct Solution {}

/// time-complexity : O(n*m)
/// space-complexity : O(n*m)
impl Solution {
    fn visit(
        grid1: &Vec<Vec<i32>>,
        grid2: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
        ok: &mut bool,
    ) {
        if visited[i][j] || grid2[i][j] != 1 {
            return;
        }
        if grid1[i][j] != 1 {
            *ok = false;
        }

        let rows = grid2.len();
        let cols = grid2[0].len();
        visited[i][j] = true;

        if i > 0 {
            Self::visit(grid1, grid2, visited, i - 1, j, ok)
        }
        if i < rows - 1 {
            Self::visit(grid1, grid2, visited, i + 1, j, ok)
        }
        if j > 0 {
            Self::visit(grid1, grid2, visited, i, j - 1, ok)
        }
        if j < cols - 1 {
            Self::visit(grid1, grid2, visited, i, j + 1, ok)
        }
    }

    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let rows = grid2.len();
        let cols = grid2[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut total = 0;
        for i in 0..rows {
            for j in 0..cols {
                if !visited[i][j] && grid2[i][j] == 1 {
                    let mut ok = true;
                    Self::visit(&grid1, &grid2, &mut visited, i, j, &mut ok);
                    if ok {
                        total += 1;
                    }
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::count_sub_islands(
                vec![
                    vec![1, 1, 1, 0, 0],
                    vec![0, 1, 1, 1, 1],
                    vec![0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 1, 1]
                ],
                vec![
                    vec![1, 1, 1, 0, 0],
                    vec![0, 0, 1, 1, 1],
                    vec![0, 1, 0, 0, 0],
                    vec![1, 0, 1, 1, 0],
                    vec![0, 1, 0, 1, 0]
                ]
            ),
            3
        );
    }
}
