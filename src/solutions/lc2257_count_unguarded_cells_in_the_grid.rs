//! <https://leetcode.com/problems/count-unguarded-cells-in-the-grid/>

struct Solution {}

/// time-complexity : O(m*n)
/// space-complexity : O(m*n)
impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        #[derive(Clone, PartialEq, Eq)]
        enum Cell {
            Unguarded,
            Guarded,
            Wall,
            Guard,
        }
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut grid = vec![vec![Cell::Unguarded; n as usize]; m as usize];

        guards
            .iter()
            .for_each(|g| grid[g[0] as usize][g[1] as usize] = Cell::Guard);
        walls
            .iter()
            .for_each(|w| grid[w[0] as usize][w[1] as usize] = Cell::Wall);

        for g in guards {
            for (ii, jj) in &directions {
                let (mut j, mut i) = (g[0], g[1]);
                'l: loop {
                    i += ii;
                    j += jj;
                    if j < 0
                        || j >= m
                        || i < 0
                        || i >= n
                        || grid[j as usize][i as usize] == Cell::Wall
                        || grid[j as usize][i as usize] == Cell::Guard
                    {
                        break 'l;
                    } else {
                        grid[j as usize][i as usize] = Cell::Guarded;
                    }
                }
            }
        }

        grid.into_iter()
            .map(|row| row.into_iter().filter(|c| c == &Cell::Unguarded).count() as i32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::count_unguarded(
                4,
                6,
                vec![vec![0, 0], vec![1, 1], vec![2, 3]],
                vec![vec![0, 1], vec![2, 2], vec![1, 4]]
            ),
            7
        );
        assert_eq!(
            Solution::count_unguarded(
                2,
                7,
                vec![vec![1, 5], vec![1, 1], vec![1, 6], vec![0, 2]],
                vec![vec![0, 6], vec![0, 3], vec![0, 5]]
            ),
            1
        );
    }
}
