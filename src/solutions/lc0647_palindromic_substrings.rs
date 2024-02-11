//! <https://leetcode.com/problems/palindromic-substrings/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n^2)
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let len = s.len();
        let mut memo = vec![vec![false; len]; len];

        for (i, mi) in memo.iter_mut().enumerate() {
            mi[i] = true;
        }

        for i in 0..len - 1 {
            if s[i] == s[i + 1] {
                memo[i][i + 1] = true;
            }
        }

        for diff in 2..len {
            for i in 0..len - diff {
                let j = i + diff;
                if s[i] == s[j] && memo[i + 1][j - 1] {
                    memo[i][j] = true;
                }
            }
        }

        memo.into_iter().flatten().filter(|&v| v).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_substrings("abc".into()), 3);
        assert_eq!(Solution::count_substrings("aaa".into()), 6);
    }
}
