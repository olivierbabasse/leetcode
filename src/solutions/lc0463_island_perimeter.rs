//! <https://leetcode.com/problems/island-perimeter/>

struct Solution {}

/// time-complexity : O(n*m)
/// space-complexity : O(1)
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let tests = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut perimeter = 0;
        for i in 0..rows {
            for j in 0..cols {
                if grid[i as usize][j as usize] == 1 {
                    let mut land = 0;
                    for t in tests {
                        if (0..rows).contains(&(i + t.0))
                            && (0..cols).contains(&(j + t.1))
                            && grid[(i + t.0) as usize][(j + t.1) as usize] == 1
                        {
                            land += 1;
                        }
                    }
                    perimeter += 4 - land;
                }
            }
        }
        perimeter
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ]),
            16
        );
    }
}
