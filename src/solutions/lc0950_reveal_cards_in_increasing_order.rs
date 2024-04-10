//! <https://leetcode.com/problems/reveal-cards-in-increasing-order/>

use std::collections::VecDeque;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable();
        let mut queue = (0..deck.len()).collect::<VecDeque<_>>();
        let mut result = vec![0; deck.len()];
        let mut i = 0;
        while let Some(q) = queue.pop_front() {
            result[q] = deck[i];
            i += 1;
            if let Some(q) = queue.pop_front() {
                queue.push_back(q);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7]),
            vec![2, 13, 3, 11, 5, 17, 7]
        );
        assert_eq!(
            Solution::deck_revealed_increasing(vec![1, 1000]),
            vec![1, 1000]
        );
    }
}
