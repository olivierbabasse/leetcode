//! <https://leetcode.com/problems/basic-calculator/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn rec(chars: &mut std::str::Chars) -> i32 {
        let mut res = 0;
        let mut cur_number = 0;
        let mut sign = 1;
        let mut in_number = false;
        loop {
            let ch = chars.next().unwrap_or(')');
            if ch.is_ascii_digit() {
                cur_number = cur_number * 10 + ch as i32 - '0' as i32;
                in_number = true;
            } else {
                if in_number {
                    res += cur_number * sign;
                    cur_number = 0;
                    sign = 1;
                    in_number = false;
                }
                match ch {
                    '(' => {
                        res += Self::rec(chars) * sign;
                        sign = 1;
                    }
                    ')' => return res,
                    '-' => sign = -sign,
                    _ => {}
                }
            }
        }
    }

    pub fn calculate(s: String) -> i32 {
        Self::rec(&mut s.chars())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::calculate("1 + 1".into()), 2);
        assert_eq!(Solution::calculate(" 2-1 + 2 ".into()), 3);
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".into()), 23);
        assert_eq!(Solution::calculate("(1)".into()), 1);
        assert_eq!(Solution::calculate("1-(     -2)".into()), 3);
    }
}
