//! <https://leetcode.com/problems/largest-odd-number-in-string/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        num.rfind(|c| ['1', '3', '5', '7', '9'].contains(&c))
            .map(|index| &num[0..=index])
            .unwrap_or_default()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::largest_odd_number("52".into()), "5".to_string());
        assert_eq!(Solution::largest_odd_number("4206".into()), "".to_string());
        assert_eq!(
            Solution::largest_odd_number("35427".into()),
            "35427".to_string()
        );
    }
}
