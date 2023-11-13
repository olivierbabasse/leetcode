//! <https://leetcode.com/problems/reverse-only-letters/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let reversed = s
            .chars()
            .filter(|&c| c.is_alphabetic())
            .rev()
            .collect::<Vec<_>>();

        let mut res = String::with_capacity(s.len());
        let mut pos = 0;
        for c in s.chars() {
            if c.is_alphabetic() {
                res.push(reversed[pos]);
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
            Solution::reverse_only_letters("ab-cd".into()),
            "dc-ba".to_string()
        );
        assert_eq!(
            Solution::reverse_only_letters("a-bC-dEf-ghIj".into()),
            "j-Ih-gfE-dCba".to_string()
        );
        assert_eq!(
            Solution::reverse_only_letters("Test1ng-Leet=code-Q!".into()),
            "Qedo1ct-eeLg=ntse-T!".to_string()
        );
    }
}
