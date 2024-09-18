//! <https://leetcode.com/problems/largest-number/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        nums.sort_unstable_by(|a, b| format!("{b}{a}").cmp(&format!("{a}{b}")));
        if nums[0] == 0 {
            "0".into()
        } else {
            nums.into_iter().map(|n| n.to_string()).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_string());
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_string()
        );
    }
}
