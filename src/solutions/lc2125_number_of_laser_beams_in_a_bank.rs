//! <https://leetcode.com/problems/number-of-laser-beams-in-a-bank/>

struct Solution {}

/// time-complexity : O(m*n)
/// space-complexity : O(1)
impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.into_iter()
            .map(|b| b.as_bytes().iter().filter(|&&c| c == b'1').count())
            .filter(|&c| c > 0)
            .collect::<Vec<_>>()
            .windows(2)
            .fold(0, |acc, w| acc + w[0] * w[1]) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::number_of_beams(vec![
                "011001".into(),
                "000000".into(),
                "010100".into(),
                "001000".into()
            ]),
            8
        );
    }
}
