//! <https://leetcode.com/problems/strange-printer/>

struct Solution {}

/// time-complexity : O(n^3)
/// space-complexity : O(n^2)
impl Solution {
    fn remove_dups(s: &str) -> String {
        let mut res = String::new();
        let mut prev = None;
        for c in s.chars() {
            if prev != Some(c) {
                res.push(c);
                prev = Some(c);
            }
        }
        res
    }

    fn rec(s: &str, start: usize, end: usize, cache: &mut Vec<Vec<Option<usize>>>) -> usize {
        if start > end {
            return 0;
        }

        if let Some(val) = cache[start][end] {
            return val;
        }

        let mut min = 1 + Self::rec(s, start + 1, end, cache);
        for k in start + 1..=end {
            if s.as_bytes()[k] == s.as_bytes()[start] {
                let min2 = Self::rec(s, start, k - 1, cache) + Self::rec(s, k + 1, end, cache);
                min = min.min(min2);
            }
        }

        cache[start][end] = Some(min);
        min
    }

    pub fn strange_printer(s: String) -> i32 {
        let s = Self::remove_dups(&s);
        let len = s.len();
        Self::rec(&s, 0, len - 1, &mut vec![vec![None; len]; len]) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::strange_printer("aaabbb".into()), 2);
        assert_eq!(Solution::strange_printer("aba".into()), 2);
    }
}
