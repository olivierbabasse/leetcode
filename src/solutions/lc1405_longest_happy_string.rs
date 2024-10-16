//! <https://leetcode.com/problems/longest-happy-string/>

use std::collections::BinaryHeap;

struct Solution {}

/// time-complexity : O(a+b+c)
/// space-complexity : O(1)
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut queue = BinaryHeap::new();
        for (i, c) in [(a, 'a'), (b, 'b'), (c, 'c')] {
            if i > 0 {
                queue.push((i, c));
            }
        }

        let mut res = String::new();
        while !queue.is_empty() {
            let (mut i, c) = queue.pop().unwrap();
            if res.len() >= 2 && res.ends_with(c) && res.chars().nth(res.len() - 2) == Some(c) {
                if queue.is_empty() {
                    break;
                }
                let (i, c) = queue.pop().unwrap();
                res.push(c);
                if i > 1 {
                    queue.push((i - 1, c));
                }
            } else {
                i -= 1;
                res.push(c);
            }
            if i > 0 {
                queue.push((i, c));
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        /*assert_eq!(
            Solution::longest_diverse_string(1, 1, 7),
            "ccbccacc".to_string()
        );*/
        assert_eq!(
            Solution::longest_diverse_string(7, 1, 0),
            "aabaa".to_string()
        );
    }
}
