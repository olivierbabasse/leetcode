//! <https://leetcode.com/problems/longest-palindromic-subsequence/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n)
impl Solution {
    fn lcs(text1: &str, text2: &str) -> i32 {
        let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
        let (len1, len2) = (text1.len(), text2.len());

        let mut prev = vec![0; len2 + 1];
        let mut cur = prev.clone();

        for i in (0..len1).rev() {
            for j in (0..len2).rev() {
                cur[j] = prev[j]
                    .max(cur[j + 1])
                    .max(prev[j + 1] + if text1[i] == text2[j] { 1 } else { 0 });
            }
            std::mem::swap(&mut prev, &mut cur)
        }

        prev[0]
    }

    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let revs: String = s.chars().rev().collect();
        Self::lcs(&s, &revs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::longest_palindrome_subseq("bbbab".into()), 4);
        assert_eq!(Solution::longest_palindrome_subseq("cbbd".into()), 2);
    }
}
