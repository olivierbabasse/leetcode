//! <https://leetcode.com/problems/determine-if-two-strings-are-close/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut freqs1 = [0; 26];
        let mut freqs2 = [0; 26];

        word1
            .as_bytes()
            .iter()
            .for_each(|b| freqs1[(b - b'a') as usize] += 1);
        word2
            .as_bytes()
            .iter()
            .for_each(|b| freqs2[(b - b'a') as usize] += 1);

        for i in 0..26 {
            if (freqs1[i] == 0) ^ (freqs2[i] == 0) {
                return false;
            }
        }

        freqs1.sort_unstable();
        freqs2.sort_unstable();
        freqs1 == freqs2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::close_strings("abc".into(), "bca".into()));
        assert!(!Solution::close_strings("a".into(), "aa".into()));
        assert!(Solution::close_strings("cabbba".into(), "abbccc".into()));
    }
}
