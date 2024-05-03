//! <https://leetcode.com/problems/compare-version-numbers/>

use std::cmp::Ordering;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1 = version1
            .split('.')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let v2 = version2
            .split('.')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        for i in 0..v1.len().max(v2.len()) {
            match v1.get(i).unwrap_or(&0).cmp(v2.get(i).unwrap_or(&0)) {
                Ordering::Greater => return 1,
                Ordering::Less => return -1,
                _ => {}
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::compare_version("1.01".into(), "1.001".into()), 0);
        assert_eq!(Solution::compare_version("1.0".into(), "1.0.0".into()), 0);
        assert_eq!(Solution::compare_version("0.1".into(), "1.1".into()), -1);
    }
}
