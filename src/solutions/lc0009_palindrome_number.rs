//! <https://leetcode.com/problems/palindrome-number/>

struct Solution {}

/// time-complexity : O(log(x))
/// space-complexity : O(log(x))
impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut digits = Vec::new();
        while x != 0 {
            digits.push(x % 10);
            x /= 10;
        }
        digits.iter().map(Clone::clone).rev().collect::<Vec<_>>() == digits
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(10));
    }
}
