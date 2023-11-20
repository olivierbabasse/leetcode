//! <https://leetcode.com/problems/minimum-amount-of-time-to-collect-garbage/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        garbage
            .into_iter()
            .rev()
            .zip(travel.into_iter().rev().chain(Some(0)))
            .fold([0, 0, 0], |[mut m, mut p, mut g], (s, d)| {
                m += s.bytes().filter(|&b| b == b'M').count() as i32;
                p += s.bytes().filter(|&b| b == b'P').count() as i32;
                g += s.bytes().filter(|&b| b == b'G').count() as i32;

                if m > 0 {
                    m += d;
                }
                if p > 0 {
                    p += d;
                }
                if g > 0 {
                    g += d;
                }

                [m, p, g]
            })
            .iter()
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::garbage_collection(
                vec!["G".into(), "P".into(), "GP".into(), "GG".into()],
                vec![2, 4, 3]
            ),
            21
        );
        assert_eq!(
            Solution::garbage_collection(
                vec!["MMM".into(), "PGM".into(), "GP".into()],
                vec![3, 10]
            ),
            37
        );
    }
}
