//! <https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/>

struct Solution {}

/// time-complexity : O(n*log(n)+n*k)
/// space-complexity : O(n*k)
impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        let len = events.len();
        let k = k as usize;
        events.sort_unstable();

        let next_event_index = events
            .iter()
            .map(|event| events.partition_point(|e| e[0] <= event[1]))
            .collect::<Vec<_>>();

        let mut dp = vec![vec![0i64; k + 1]; len + 1];
        for index in (0..len).rev() {
            for curk in 1..=k {
                dp[index][curk] = dp[index + 1][curk]
                    .max(events[index][2] as i64 + dp[next_event_index[index]][curk - 1]);
            }
        }
        dp[0][k] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]], 2),
            7
        );
        assert_eq!(
            Solution::max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]], 2),
            10
        );
        assert_eq!(
            Solution::max_value(
                vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![4, 4, 4]],
                3
            ),
            9
        );
        let mut events = Vec::new();
        for i in 1..=100000 {
            events.push(vec![i, i, i]);
        }
        assert_eq!(Solution::max_value(events, 10), 999955);
    }
}
