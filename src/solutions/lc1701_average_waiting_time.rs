//! <https://leetcode.com/problems/average-waiting-time/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut next = 0;
        let mut wait = 0.0;
        for c in &customers {
            next = next.max(c[0]) + c[1];
            wait += (next - c[0]) as f64;
        }
        wait / customers.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]]),
            5.0
        );
        assert_eq!(
            Solution::average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]]),
            3.25
        );
    }
}
