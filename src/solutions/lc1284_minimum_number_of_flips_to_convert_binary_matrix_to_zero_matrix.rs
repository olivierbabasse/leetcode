//! <https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix/>

use std::collections::{HashSet, VecDeque};

struct Solution {}

/// time-complexity : O(2^(n*m))
/// space-complexity : O(n*m)
impl Solution {
    fn flip(mat: &mut [Vec<i32>], i: usize, j: usize) {
        for (di, dj) in [(1, 0), (-1, 0), (0, 0), (0, 1), (0, -1)] {
            let (ni, nj) = (i as i32 + di, j as i32 + dj);
            if ni >= 0 && nj >= 0 && ni < mat.len() as i32 && nj < mat[0].len() as i32 {
                mat[ni as usize][nj as usize] ^= 1;
            }
        }
    }

    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(mat);
        while !queue.is_empty() {
            let qlen = queue.len();
            for _ in 0..qlen {
                let cur = queue.pop_front().unwrap();
                if !visited.insert(cur.clone()) {
                    continue;
                }

                let mut done = true;
                for i in 0..cur.len() {
                    for j in 0..cur[0].len() {
                        if cur[i][j] == 1 {
                            done = false;
                        }

                        let mut c = cur.clone();
                        Self::flip(&mut c, i, j);
                        queue.push_back(c);
                    }
                }

                if done {
                    return res;
                }
            }
            res += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_flips(vec![vec![0, 0], vec![0, 1]]), 3);
        assert_eq!(Solution::min_flips(vec![vec![0]]), 0);
        assert_eq!(Solution::min_flips(vec![vec![1, 0, 0], vec![1, 0, 0]]), -1);
    }
}
