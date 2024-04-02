//! <https://leetcode.com/problems/isomorphic-strings/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut f1 = [0; 256];
        let mut f2 = [0; 256];
        for (&s, &t) in s.as_bytes().iter().zip(t.as_bytes()) {
            if f1[s as usize] == 0 && f2[t as usize] == 0 {
                f1[s as usize] = t;
                f2[t as usize] = s;
            } else if f1[s as usize] != t || f2[t as usize] != s {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert!(Solution::is_isomorphic("egg".into(), "add".into()));
        assert!(!Solution::is_isomorphic("foo".into(), "bar".into()));
        assert!(Solution::is_isomorphic("paper".into(), "title".into()));
        assert!(!Solution::is_isomorphic(
            "bbbaaaba".into(),
            "aaabbbba".into()
        ));
    }
}
