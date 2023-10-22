//! <https://leetcode.com/problems/roman-to-integer/>

struct Solution {}

/// time-complexity : O(len(s))
/// space-complexity : O(len(s))
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<_>>();
        let mut index = 0;
        let mut number = 0;
        while index < chars.len() {
            let mut double = false;
            if index < chars.len() - 1 {
                double = true;
                match (chars[index], chars[index + 1]) {
                    ('I', 'V') => number += 4,
                    ('I', 'X') => number += 9,
                    ('X', 'L') => number += 40,
                    ('X', 'C') => number += 90,
                    ('C', 'D') => number += 400,
                    ('C', 'M') => number += 900,
                    _ => double = false,
                }
            }
            if !double {
                match chars[index] {
                    'I' => number += 1,
                    'V' => number += 5,
                    'X' => number += 10,
                    'L' => number += 50,
                    'C' => number += 100,
                    'D' => number += 500,
                    'M' => number += 1000,
                    _ => {}
                }
                index += 1;
            } else {
                index += 2;
            }
        }
        number
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::roman_to_int("III".into()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".into()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994);
    }
}
