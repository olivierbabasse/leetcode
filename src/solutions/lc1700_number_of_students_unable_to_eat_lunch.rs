//! <https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/>

use std::collections::VecDeque;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut sandwiches: Vec<_> = sandwiches.into_iter().rev().collect();
        let mut students: VecDeque<_> = students.into_iter().collect();

        let mut count = 0;
        while !students.is_empty() && !sandwiches.is_empty() && count <= students.len() {
            if students.front().unwrap() == sandwiches.last().unwrap() {
                count = 0;
                students.pop_front();
                sandwiches.pop();
            } else {
                let s = students.pop_front().unwrap();
                students.push_back(s);
                count += 1;
            }
        }

        students.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        );
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}
