//! <https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/>

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    fn rec(arr: &[String], index: usize, mask: [bool; 26]) -> i32 {
        if index >= arr.len() {
            return 0;
        }

        let mut res = 0;
        let mut newmask = mask;
        let mut collision = false;
        for c in arr[index].chars() {
            if newmask[c as usize - 'a' as usize] {
                collision = true;
            }
            newmask[c as usize - 'a' as usize] = true;
        }
        if !collision {
            res = arr[index].len() as i32 + Self::rec(arr, index + 1, newmask);
        }

        res.max(Self::rec(arr, index + 1, mask))
    }

    pub fn max_length(arr: Vec<String>) -> i32 {
        Self::rec(&arr, 0, [false; 26])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_length(vec!["un".into(), "iq".into(), "ue".into()]),
            4
        );
        assert_eq!(
            Solution::max_length(vec!["cha".into(), "r".into(), "act".into(), "ers".into()]),
            6
        );
        assert_eq!(
            Solution::max_length(vec!["abcdefghijklmnopqrstuvwxyz".into()]),
            26
        );
        assert_eq!(Solution::max_length(vec!["aa".into(), "bb".into()]), 0);
    }
}
