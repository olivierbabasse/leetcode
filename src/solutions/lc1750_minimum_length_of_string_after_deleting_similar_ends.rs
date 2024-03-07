//! <https://leetcode.com/problems/minimum-length-of-string-after-deleting-similar-ends/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j && s.as_bytes()[i] == s.as_bytes()[j] {
            let c = s.as_bytes()[i];

            while i <= j && s.as_bytes()[i] == c {
                i += 1;
            }

            while j > i && s.as_bytes()[j] == c {
                j -= 1;
            }
        }
        j as i32 - i as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::minimum_length("ca".into()), 2);
        assert_eq!(Solution::minimum_length("cabaabac".into()), 0);
        assert_eq!(Solution::minimum_length("aabccabba".into()), 3);
    }
}
