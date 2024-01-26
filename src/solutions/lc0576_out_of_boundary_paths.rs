//! <https://leetcode.com/problems/out-of-boundary-paths/>

struct Solution {}

/// time-complexity : O(m*n*max_move)
/// space-complexity : O(m*n*max_move)
impl Solution {
    fn rec(
        m: i32,
        n: i32,
        max_move: i32,
        start_row: i32,
        start_column: i32,
        cache: &mut [[[Option<i32>; 51]; 50]; 50],
    ) -> i32 {
        if start_column < 0 || start_column >= n || start_row < 0 || start_row >= m {
            1
        } else if max_move == 0 {
            0
        } else if let Some(res) =
            cache[start_row as usize][start_column as usize][max_move as usize]
        {
            res
        } else {
            let mut res = (Self::rec(m, n, max_move - 1, start_row - 1, start_column, cache)
                + Self::rec(m, n, max_move - 1, start_row + 1, start_column, cache))
                % 1000000007
                + (Self::rec(m, n, max_move - 1, start_row, start_column - 1, cache)
                    + Self::rec(m, n, max_move - 1, start_row, start_column + 1, cache))
                    % 1000000007;
            res %= 1000000007;
            cache[start_row as usize][start_column as usize][max_move as usize] = Some(res);
            res
        }
    }

    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut cache = [[[None; 51]; 50]; 50];
        Self::rec(m, n, max_move, start_row, start_column, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
        assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
        assert_eq!(Solution::find_paths(2, 3, 8, 1, 0), 1104);
        assert_eq!(Solution::find_paths(4, 5, 8, 3, 2), 3875);
        assert_eq!(Solution::find_paths(36, 5, 50, 15, 3), 390153306);
    }
}
