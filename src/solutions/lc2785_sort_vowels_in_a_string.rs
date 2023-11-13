//! <https://leetcode.com/problems/sort-vowels-in-a-string/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowels = Vec::from(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
        let mut sorted_vowels = s
            .chars()
            .filter(|&c| vowels.iter().any(|&v| v == c))
            .collect::<Vec<_>>();
        sorted_vowels.sort_unstable();

        let mut res = String::with_capacity(s.len());
        let mut pos = 0;
        for c in s.chars() {
            if vowels.iter().any(|&v| v == c) {
                res.push(sorted_vowels[pos]);
                pos += 1;
            } else {
                res.push(c);
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
        assert_eq!(
            Solution::sort_vowels("lEetcOde".into()),
            "lEOtcede".to_string()
        );
        assert_eq!(Solution::sort_vowels("lYmpH".into()), "lYmpH".to_string());
    }
}
