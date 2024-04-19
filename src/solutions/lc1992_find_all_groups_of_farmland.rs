//! <https://leetcode.com/problems/find-all-groups-of-farmland/>

struct Solution {}

/// time-complexity : O(n*m)
/// space-complexity : O(n*m)
impl Solution {
    fn visit(
        land: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
        maxi: &mut usize,
        maxj: &mut usize,
    ) {
        if visited[i][j] || land[i][j] != 1 {
            return;
        }

        let rows = land.len();
        let cols = land[0].len();
        visited[i][j] = true;

        *maxi = i.max(*maxi);
        *maxj = j.max(*maxj);

        if i > 0 {
            Self::visit(land, visited, i - 1, j, maxi, maxj);
        }
        if i < rows - 1 {
            Self::visit(land, visited, i + 1, j, maxi, maxj);
        }
        if j > 0 {
            Self::visit(land, visited, i, j - 1, maxi, maxj);
        }
        if j < cols - 1 {
            Self::visit(land, visited, i, j + 1, maxi, maxj);
        }
    }

    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = land.len();
        let cols = land[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut res = Vec::new();
        for i in 0..rows {
            for j in 0..cols {
                if !visited[i][j] && land[i][j] == 1 {
                    let (mut maxi, mut maxj) = (i, j);
                    Self::visit(&land, &mut visited, i, j, &mut maxi, &mut maxj);
                    res.push(vec![i as i32, j as i32, maxi as i32, maxj as i32]);
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_farmland(vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]]),
            vec![vec![0, 0, 0, 0], vec![1, 1, 2, 2]]
        );
    }
}
