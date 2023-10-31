//! <https://leetcode.com/problems/find-the-original-array-of-prefix-xor/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut a = 0;
        pref.into_iter()
            .map(|e| {
                let tmp = a ^ e;
                a ^= tmp;
                tmp
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_array(vec![5, 2, 0, 3, 1]),
            vec![5, 7, 2, 3, 2]
        );
        assert_eq!(Solution::find_array(vec![13]), vec![13]);
    }
}
