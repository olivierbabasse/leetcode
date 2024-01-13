//! <https://leetcode.com/problems/largest-palindromic-number/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut freqs = [0; 10];
        num.as_bytes()
            .iter()
            .for_each(|b| freqs[(b - b'0') as usize] += 1);

        let mut prefix = String::new();
        let mut rsuffix = String::new();
        for i in (0..10).rev() {
            if i == 0 && prefix.is_empty() {
                break;
            }
            while freqs[i] >= 2 {
                let c = ('0' as usize + i) as u8 as char;
                prefix.push(c);
                rsuffix.push(c);
                freqs[i] -= 2;
            }
        }
        for i in (0..10).rev() {
            if freqs[i] >= 1 {
                let c = ('0' as usize + i) as u8 as char;
                prefix.push(c);
                break;
            }
        }
        for c in rsuffix.chars().rev() {
            prefix.push(c)
        }

        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::largest_palindromic("444947137".into()),
            "7449447".to_string()
        );
        assert_eq!(
            Solution::largest_palindromic("00009".into()),
            "9".to_string()
        );
    }
}
