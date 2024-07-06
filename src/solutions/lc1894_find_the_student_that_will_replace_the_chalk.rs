//! <https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut left = k as i64 % chalk.iter().map(|i| *i as i64).sum::<i64>();
        for (i, ch) in chalk.into_iter().enumerate() {
            if left < ch as i64 {
                return i as i32;
            } else {
                left -= ch as i64;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::chalk_replacer(vec![5, 1, 5], 22), 0);
        assert_eq!(Solution::chalk_replacer(vec![3, 4, 1, 2], 25), 1);
    }
}
