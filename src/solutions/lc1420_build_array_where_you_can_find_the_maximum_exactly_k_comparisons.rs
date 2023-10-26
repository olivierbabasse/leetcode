//! <https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n*m*k)
/// space-complexity : O(n*m*k)
impl Solution {
    fn solve(
        n: i32,
        m: i32,
        k: i32,
        pos: i32,
        max: i32,
        distinct: i32,
        memo: &mut HashMap<(i32, i32, i32), i32>,
    ) -> i32 {
        if pos == n {
            if distinct == k {
                return 1;
            }
            return 0;
        }

        if let Some(res) = memo.get(&(pos, max, distinct)) {
            return *res;
        }

        let mut count = 0;
        for i in 1..=m {
            count = (count
                + if i > max {
                    Self::solve(n, m, k, pos + 1, i, distinct + 1, memo)
                } else {
                    Self::solve(n, m, k, pos + 1, max, distinct, memo)
                })
                % 1000000007;
        }

        memo.insert((pos, max, distinct), count);
        count
    }

    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        Self::solve(n, m, k, 0, -1, 0, &mut HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_of_arrays(2, 3, 1), 6);
        assert_eq!(Solution::num_of_arrays(5, 2, 3), 0);
        assert_eq!(Solution::num_of_arrays(9, 1, 1), 1);
    }
}
