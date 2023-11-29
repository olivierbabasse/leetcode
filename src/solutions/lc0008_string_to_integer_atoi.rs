//! <https://leetcode.com/problems/string-to-integer-atoi/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut res: i32 = 0;
        let mut neg = false;
        let mut pos = false;
        let mut digits = false;
        for &c in s.as_bytes() {
            if c == b' ' {
                if digits || pos || neg {
                    break;
                }
            } else if (c == b'+' || c == b'-') && !digits {
                if pos || neg {
                    break;
                }
                if c == b'+' {
                    pos = true
                } else if c == b'-' {
                    neg = true
                }
            } else if c.is_ascii_digit() {
                digits = true;
                res = res.saturating_mul(10);
                let d = (c - b'0') as i32;
                res = res.saturating_add(if neg { -d } else { d });
            } else {
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::my_atoi("42".into()), 42);
        assert_eq!(Solution::my_atoi("   -42".into()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".into()), 4193);
        assert_eq!(Solution::my_atoi("words and 987".into()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".into()), -2147483648);
        assert_eq!(Solution::my_atoi("+1".into()), 1);
        assert_eq!(Solution::my_atoi("+-12".into()), 0);
        assert_eq!(Solution::my_atoi("00000-42a1234".into()), 0);
        assert_eq!(Solution::my_atoi("   +0 123".into()), 0);
        assert_eq!(Solution::my_atoi("-2147483647".into()), -2147483647);
        assert_eq!(Solution::my_atoi("-5-".into()), -5);
        assert_eq!(Solution::my_atoi("-13+8".into()), -13);
        assert_eq!(Solution::my_atoi("  +  413".into()), 0);
        assert_eq!(Solution::my_atoi(" ++1".into()), 0);
    }
}
