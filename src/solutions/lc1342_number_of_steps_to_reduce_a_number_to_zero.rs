//! <https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/>

struct Solution {}

impl Solution {
    /// time-complexity : O(n)
    /// space-complexity : O(1)
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut count = 0;
        while num > 0 {
            num = match num % 2 {
                0 => num / 2,
                _ => num - 1,
            };
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::number_of_steps(14), 6);
        assert_eq!(Solution::number_of_steps(8), 4);
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}
