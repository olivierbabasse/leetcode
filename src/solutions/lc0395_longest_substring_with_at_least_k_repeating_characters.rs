//! <https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n)
impl Solution {
    fn rec(s: &str, k: i32) -> i32 {
        let b = s.as_bytes();
        let mut freqs = [0; 26];
        for &c in b {
            freqs[(c - b'a') as usize] += 1;
        }
        for i in 0..s.len() {
            if freqs[(b[i] - b'a') as usize] < k {
                return Self::rec(&s[0..i], k).max(Self::rec(&s[i + 1..], k));
            }
        }
        s.len() as i32
    }

    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::rec(&s, k)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::longest_substring("aaabb".into(), 3), 3);
        assert_eq!(Solution::longest_substring("ababbc".into(), 2), 5);
    }
}
