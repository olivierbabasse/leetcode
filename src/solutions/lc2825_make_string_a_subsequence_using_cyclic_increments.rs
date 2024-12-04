//! <https://leetcode.com/problems/make-string-a-subsequence-using-cyclic-increments/>

struct Solution {}

/// time-complexity : O(len(str1)+len(str2))
/// space-complexity : O(1)
impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let (str1, str2) = (str1.as_bytes(), str2.as_bytes());
        let (mut index1, mut index2) = (0, 0);

        while index1 < str1.len() && index2 < str2.len() {
            if str1[index1] == str2[index2]
                || str1[index1] + 1 == str2[index2]
                || str1[index1] == str2[index2] + 25
            {
                index2 += 1;
            }
            index1 += 1;
        }

        index2 == str2.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::can_make_subsequence("abc".into(), "ad".into()));
        assert!(Solution::can_make_subsequence("zc".into(), "ad".into()));
        assert!(!Solution::can_make_subsequence("ab".into(), "d".into()));
    }
}
