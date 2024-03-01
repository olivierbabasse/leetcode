//! <https://leetcode.com/problems/maximum-odd-binary-number/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let (mut zeros, mut ones) = (0, 0);
        for &b in s.as_bytes() {
            if b == b'0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        let mut res = String::new();
        for _ in 0..ones - 1 {
            res.push('1');
        }
        for _ in 0..zeros {
            res.push('0');
        }
        res.push('1');
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::maximum_odd_binary_number("010".into()),
            "001".to_string()
        );
        assert_eq!(
            Solution::maximum_odd_binary_number("0101".into()),
            "1001".to_string()
        );
    }
}
