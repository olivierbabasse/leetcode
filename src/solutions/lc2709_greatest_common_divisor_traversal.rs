//! <https://leetcode.com/problems/greatest-common-divisor-traversal/>

use std::collections::HashSet;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        const MAX: usize = 100004;

        if nums.len() == 1 && nums[0] == 1 {
            return true;
        }

        let mut primes = [true; MAX];
        primes[0] = false;
        primes[1] = false;
        let mut i = 2;
        while i * i <= MAX {
            if primes[i] {
                for j in (i * i..MAX).step_by(i) {
                    primes[j] = false
                }
            }
            i += 1;
        }
        let primes = primes
            .into_iter()
            .enumerate()
            .filter(|(_, p)| *p)
            .map(|(i, _)| i as i32)
            .collect::<Vec<_>>();

        let mut visited = [false; MAX];
        let mut adj: Vec<HashSet<i32>> = vec![HashSet::new(); MAX];
        for n in &nums {
            if visited[*n as usize] {
                continue;
            }
            visited[*n as usize] = true;

            let mut val = *n;
            let mut pi = 0;
            while val >= 1 && primes[pi] <= val {
                if val % primes[pi] == 0 {
                    adj[*n as usize].insert(primes[pi]);
                    adj[primes[pi] as usize].insert(*n);

                    while val % primes[pi] == 0 {
                        val /= primes[pi];
                    }
                }
                pi += 1;
            }
        }

        let mut visited = [false; MAX];
        let mut stack = Vec::new();
        stack.push(nums[0]);
        while let Some(n) = stack.pop() {
            for x in &adj[n as usize] {
                if !visited[*x as usize] {
                    visited[*x as usize] = true;
                    stack.push(*x);
                }
            }
        }

        nums.iter().all(|n| visited[*n as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::can_traverse_all_pairs(vec![
            99991, 99991, 99991, 99991, 99991
        ]));
        assert!(Solution::can_traverse_all_pairs(vec![1]));
        assert!(Solution::can_traverse_all_pairs(vec![2, 3, 6]));
        assert!(!Solution::can_traverse_all_pairs(vec![3, 9, 5]));
        assert!(Solution::can_traverse_all_pairs(vec![4, 3, 12, 8]));
    }
}
