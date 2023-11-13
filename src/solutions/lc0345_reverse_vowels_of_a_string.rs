//! <https://leetcode.com/problems/reverse-vowels-of-a-string/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = Vec::from(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
        let filtered = s
            .chars()
            .filter(|&c| vowels.iter().any(|&v| v == c))
            .collect::<Vec<_>>();

        let mut res = String::with_capacity(s.len());
        let mut pos = filtered.len();
        for c in s.chars() {
            if vowels.iter().any(|&v| v == c) {
                pos -= 1;
                res.push(filtered[pos]);
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
            Solution::reverse_vowels("hello".into()),
            "holle".to_string()
        );
        assert_eq!(
            Solution::reverse_vowels("leetcode".into()),
            "leotcede".to_string()
        );
    }
}
