//! <https://leetcode.com/problems/faulty-keyboard/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn final_string(s: String) -> String {
        let mut res = String::new();
        for c in s.chars() {
            if c == 'i' {
                res = res.chars().rev().collect();
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
        assert_eq!(Solution::final_string("string".into()), "rtsng".to_string());
        assert_eq!(
            Solution::final_string("poiinter".into()),
            "ponter".to_string()
        );
    }
}
