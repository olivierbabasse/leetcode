//! <https://leetcode.com/problems/sliding-puzzle/>

use std::collections::{HashSet, VecDeque};

struct Solution {}

/// time-complexity : O(n!)
/// space-complexity : O(n!)
impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let directions: Vec<Vec<usize>> = vec![
            vec![1, 3],
            vec![0, 2, 4],
            vec![1, 5],
            vec![0, 4],
            vec![1, 3, 5],
            vec![2, 4],
        ];

        let target = vec![1u8, 2, 3, 4, 5, 0];
        let start = board
            .concat()
            .into_iter()
            .map(|i| i as u8)
            .collect::<Vec<_>>();

        let mut count = 0;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                let current_state = queue.pop_front().unwrap();
                if current_state == target {
                    return count;
                }
                let zero_pos = current_state.iter().position(|&x| x == 0).unwrap();
                for new_pos in &directions[zero_pos] {
                    let mut next_state = current_state.clone();
                    next_state.swap(zero_pos, *new_pos);
                    if visited.contains(&next_state) {
                        continue;
                    }
                    visited.insert(next_state.clone());
                    queue.push_back(next_state);
                }
            }
            count += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![4, 0, 5]]),
            1
        );
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![5, 4, 0]]),
            -1
        );
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![4, 1, 2], vec![5, 0, 3]]),
            5
        );
    }
}
