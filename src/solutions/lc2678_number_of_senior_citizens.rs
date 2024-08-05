//! <https://leetcode.com/problems/number-of-senior-citizens/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .into_iter()
            .map(|s| s[11..13].parse::<i32>().unwrap())
            .filter(|&a| a > 60)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::count_seniors(vec![
                "7868190130M7522".into(),
                "5303914400F9211".into(),
                "9273338290F4010".into()
            ]),
            2
        );
        assert_eq!(
            Solution::count_seniors(vec!["1313579440F2036".into(), "2921522980M5644".into(),]),
            0
        );
    }
}
