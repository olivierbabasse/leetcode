//! <https://leetcode.com/problems/isomorphic-strings/>

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut f1 = [0; 26];
        let mut f2 = [0; 26];
        s.as_bytes()
            .iter()
            .for_each(|&b| f1[(b - b'a') as usize] += 1);
        t.as_bytes()
            .iter()
            .for_each(|&b| f2[(b - b'a') as usize] += 1);
        f1.sort_unstable();
        f2.sort_unstable();
        f1 == f2
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
    }
}
