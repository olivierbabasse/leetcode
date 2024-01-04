//! <https://leetcode.com/problems/meeting-rooms-iii/>

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}

/// time-complexity : O(m*log(m))
/// space-complexity : O(n+m)
impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        meetings.sort_unstable();

        let mut meeting_count_by_room = vec![0usize; n];
        let mut busy_rooms: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        let mut free_rooms = (0..n).map(Reverse).collect::<BinaryHeap<_>>();

        for m in meetings {
            let start = m[0] as i64;
            let stop = m[1] as i64;

            while busy_rooms.peek().is_some() && busy_rooms.peek().unwrap().0 .0 <= start {
                let Reverse((_, room)) = busy_rooms.pop().unwrap();
                free_rooms.push(Reverse(room));
            }

            if let Some(Reverse(room)) = free_rooms.pop() {
                busy_rooms.push(Reverse((stop, room)));
                meeting_count_by_room[room] += 1;
            } else if let Some(Reverse((end, room))) = busy_rooms.pop() {
                busy_rooms.push(Reverse((end + stop - start, room)));
                meeting_count_by_room[room] += 1;
            }
        }

        meeting_count_by_room
            .into_iter()
            .enumerate()
            .max_by_key(|(pos, count)| (*count, Reverse(*pos)))
            .map(|(pos, _)| pos)
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]]),
            0
        );
        assert_eq!(
            Solution::most_booked(
                3,
                vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]]
            ),
            1
        );
    }
}
