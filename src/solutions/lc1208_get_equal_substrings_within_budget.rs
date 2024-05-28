//! <https://leetcode.com/problems/get-equal-substrings-within-budget/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let diffs = s
            .as_bytes()
            .iter()
            .zip(t.as_bytes())
            .map(|(&s, &t)| (s as i32 - t as i32).abs())
            .collect::<Vec<_>>();
        let (mut max, mut current, mut begin) = (0, 0, 0);
        for end in 0..diffs.len() {
            current += diffs[end];
            while current > max_cost {
                current -= diffs[begin];
                begin += 1;
            }
            max = max.max(end as i32 - begin as i32 + 1);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::equal_substring("abcd".into(), "bcdf".into(), 3),
            3
        );
        assert_eq!(
            Solution::equal_substring("abcd".into(), "cdef".into(), 3),
            1
        );
        assert_eq!(
            Solution::equal_substring("abcd".into(), "acde".into(), 0),
            1
        );
    }
}
