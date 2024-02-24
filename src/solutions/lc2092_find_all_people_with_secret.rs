//! <https://leetcode.com/problems/find-all-people-with-secret/>

use std::collections::{HashMap, VecDeque};

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let mut adj: HashMap<i32, Vec<_>> = HashMap::new();
        for m in meetings {
            adj.entry(m[0]).or_default().push((m[1], m[2]));
            adj.entry(m[1]).or_default().push((m[0], m[2]));
        }

        let mut learn_secret = vec![None; n as usize];
        learn_secret[0] = Some(0);
        learn_secret[first_person as usize] = Some(0);

        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        queue.push_back((first_person, 0));
        while let Some((cur_person, cur_time)) = queue.pop_front() {
            for (p, t) in adj.get(&cur_person).unwrap_or(&vec![]) {
                if t >= &cur_time && learn_secret[*p as usize].unwrap_or(i32::MAX) > *t {
                    learn_secret[*p as usize] = Some(*t);
                    queue.push_back((*p, *t));
                }
            }
        }

        learn_secret
            .iter()
            .enumerate()
            .filter(|(_, &t)| t.is_some())
            .map(|(i, _)| i as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_all_people(6, vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]], 1),
            vec![0, 1, 2, 3, 5]
        );
        assert_eq!(
            Solution::find_all_people(4, vec![vec![3, 1, 3], vec![1, 2, 2], vec![0, 3, 3]], 3),
            vec![0, 1, 3]
        );
        assert_eq!(
            Solution::find_all_people(5, vec![vec![3, 4, 2], vec![1, 2, 1], vec![2, 3, 1]], 1),
            vec![0, 1, 2, 3, 4]
        );
    }
}
