//! <https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let len = boxes.len();
        let mut res = vec![0; len];
        for (i, res) in res.iter_mut().enumerate() {
            for (j, c) in boxes.as_bytes().iter().enumerate() {
                if *c == b'1' {
                    *res += (i as i32 - j as i32).abs();
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_operations("110".into()), vec![1, 1, 3]);
        assert_eq!(
            Solution::min_operations("001011".into()),
            vec![11, 8, 5, 4, 3, 4]
        );
    }
}
