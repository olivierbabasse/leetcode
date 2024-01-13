//! <https://leetcode.com/problems/determine-if-string-halves-are-alike/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    fn vowels(s: &[u8]) -> usize {
        let vowels = [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];
        s.iter().filter(|&b| vowels.contains(b)).count()
    }

    pub fn halves_are_alike(s: String) -> bool {
        let n = s.len() / 2;
        Self::vowels(&s.as_bytes()[0..n]) == Self::vowels(&s.as_bytes()[n..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::halves_are_alike("book".into()));
        assert!(!Solution::halves_are_alike("textbook".into()));
    }
}
