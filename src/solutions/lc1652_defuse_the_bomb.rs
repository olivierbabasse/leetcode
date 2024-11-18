//! <https://leetcode.com/problems/defuse-the-bomb/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![0; code.len()];
        let (mut start, mut end) = (1, k as usize);
        if k < 0 {
            start = (code.len() as i32 + k) as usize;
            end = code.len() - 1;
        }
        let mut sum = code[start..=end].iter().sum();
        for r in res.iter_mut() {
            *r = sum;
            sum -= code[start % code.len()];
            sum += code[(end + 1) % code.len()];
            (start, end) = (start + 1, end + 1)
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
        assert_eq!(Solution::decrypt(vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0]);
        assert_eq!(Solution::decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
    }
}
