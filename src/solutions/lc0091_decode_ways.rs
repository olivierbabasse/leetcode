//! <https://leetcode.com/problems/decode-ways/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn rec(s: &[u8], index: usize, cache: &mut [Option<i32>]) -> i32 {
        if let Some(val) = cache[index] {
            return val;
        }
        let len = s.len();
        if index == len {
            return 1;
        }
        if s[index] == b'0' {
            return 0;
        }
        let mut count = Self::rec(s, index + 1, cache);
        if index + 1 < len {
            let v = (s[index] - b'0') * 10 + (s[index + 1] - b'0');
            if (10..=26).contains(&v) {
                count += Self::rec(s, index + 2, cache);
            }
        }
        cache[index] = Some(count);
        count
    }

    pub fn num_decodings(s: String) -> i32 {
        Self::rec(s.as_bytes(), 0, &mut vec![None; s.len() + 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_decodings("12".into()), 2);
        assert_eq!(Solution::num_decodings("226".into()), 3);
        assert_eq!(Solution::num_decodings("06".into()), 0);
        assert_eq!(
            Solution::num_decodings("111111111111111111111111111111111111111111111".into()),
            1836311903
        );
    }
}
