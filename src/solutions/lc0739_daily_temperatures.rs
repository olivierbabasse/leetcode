//! <https://leetcode.com/problems/daily-temperatures/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let len = temperatures.len();
        let mut res = vec![0; len];
        let mut stack = Vec::new();

        for i in (0..len).rev() {
            while !stack.is_empty() && temperatures[i] >= temperatures[*stack.last().unwrap()] {
                stack.pop();
            }
            res[i] = (*stack.last().unwrap_or(&i) - i) as i32;
            stack.push(i);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }
}
