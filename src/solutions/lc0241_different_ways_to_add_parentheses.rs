//! <https://leetcode.com/problems/different-ways-to-add-parentheses/>

struct Solution {}

/// time-complexity : O(n*2^n)
/// space-complexity : O(n^2*2^n)
impl Solution {
    fn rec(
        expression: &str,
        begin: usize,
        end: usize,
        cache: &mut Vec<Vec<Option<Vec<i32>>>>,
    ) -> Vec<i32> {
        if let Some(res) = cache[begin][end].as_ref() {
            return res.clone();
        }

        if begin == end {
            return vec![(expression.as_bytes()[begin] - b'0') as i32];
        }

        if begin + 1 == end && expression.as_bytes()[begin].is_ascii_digit() {
            let a = (expression.as_bytes()[begin] - b'0') as i32;
            let b = (expression.as_bytes()[end] - b'0') as i32;
            return vec![a * 10 + b];
        }

        let mut res = Vec::new();
        for i in begin..=end {
            if expression.as_bytes()[i].is_ascii_digit() {
                continue;
            }

            let left = Self::rec(expression, begin, i - 1, cache);
            let right = Self::rec(expression, i + 1, end, cache);

            for l in &left {
                for r in &right {
                    match expression.as_bytes()[i] {
                        b'+' => res.push(l + r),
                        b'-' => res.push(l - r),
                        b'*' => res.push(l * r),
                        _ => {}
                    }
                }
            }
        }

        cache[begin][end] = Some(res.clone());
        res
    }

    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let len = expression.len();
        Self::rec(&expression, 0, len - 1, &mut vec![vec![None; len]; len])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::array_eq;

    #[test]
    fn test_cases() {
        assert!(array_eq(
            &Solution::diff_ways_to_compute("2-1-1".into()),
            &[0, 2]
        ));
        assert!(array_eq(
            &Solution::diff_ways_to_compute("2*3-4*5".into()),
            &[-34, -14, -10, -10, 10]
        ));
    }
}
