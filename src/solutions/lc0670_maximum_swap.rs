//! <https://leetcode.com/problems/maximum-swap/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn maximum_swap(mut num: i32) -> i32 {
        if num < 10 {
            return num;
        }

        let mut digits: Vec<i32> = Vec::new();
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();

        let len = digits.len();
        let mut max_right_index = vec![0usize; len];
        max_right_index[len - 1] = len - 1;
        for i in (0..=len - 2).rev() {
            max_right_index[i] = if digits[i] > digits[max_right_index[i + 1]] {
                i
            } else {
                max_right_index[i + 1]
            }
        }

        for i in 0..len {
            if digits[i] < digits[max_right_index[i]] {
                digits.swap(i, max_right_index[i]);
                break;
            }
        }

        digits.into_iter().fold(0, |acc, v| acc * 10 + v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::maximum_swap(2736), 7236);
        assert_eq!(Solution::maximum_swap(9973), 9973);
    }
}
