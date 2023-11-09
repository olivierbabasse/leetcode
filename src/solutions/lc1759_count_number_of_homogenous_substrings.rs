//! <https://leetcode.com/problems/count-number-of-homogenous-substrings/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let mut current_char = s.chars().nth(0).unwrap();
        let mut current_count = 1i64;
        let mut total = 0i64;
        for c in s.chars().skip(1) {
            if c != current_char {
                total += current_count * (current_count + 1) / 2;
                current_char = c;
                current_count = 0;
            }
            current_count += 1;
        }
        ((total + current_count * (current_count + 1) / 2) % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_homogenous("abbcccaa".into()), 13);
        assert_eq!(Solution::count_homogenous("xy".into()), 2);
        assert_eq!(Solution::count_homogenous("zzzzz".into()), 15);
    }
}
