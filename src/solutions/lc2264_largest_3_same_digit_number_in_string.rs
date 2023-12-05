//! <https://leetcode.com/problems/largest-3-same-digit-number-in-string/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(n)
impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        num.as_bytes()
            .windows(3)
            .filter_map(|w| {
                if w[0] == w[1] && w[1] == w[2] {
                    Some(w[0])
                } else {
                    None
                }
            })
            .max()
            .map(|m| vec![m as char, m as char, m as char].into_iter().collect())
            .unwrap_or_else(|| "".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::largest_good_integer("6777133339".into()),
            "777".to_string()
        );
        assert_eq!(
            Solution::largest_good_integer("2300019".into()),
            "000".to_string()
        );
        assert_eq!(
            Solution::largest_good_integer("42352338".into()),
            "".to_string()
        );
    }
}
