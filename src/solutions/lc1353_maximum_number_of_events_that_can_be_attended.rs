//! <https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/>

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable();
        let len = events.len();

        let mut day = 0;
        let mut count = 0;
        let mut queue: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut index = 0;

        while !queue.is_empty() || index < len {
            if queue.is_empty() {
                day = events[index][0];
            }

            while index < len && events[index][0] <= day {
                queue.push(Reverse(events[index][1]));
                index += 1;
            }
            queue.pop();

            day += 1;
            count += 1;

            while !queue.is_empty() && queue.peek().unwrap().0 < day {
                queue.pop();
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
            3
        );
        assert_eq!(
            Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]),
            4
        );
        assert_eq!(
            Solution::max_events(vec![
                vec![1, 4],
                vec![4, 4],
                vec![2, 2],
                vec![3, 4],
                vec![1, 1]
            ]),
            4
        );
        assert_eq!(
            Solution::max_events(vec![
                vec![1, 2],
                vec![1, 2],
                vec![3, 3],
                vec![1, 5],
                vec![1, 5]
            ]),
            5
        );
    }
}
