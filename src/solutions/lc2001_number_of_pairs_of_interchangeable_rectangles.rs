//! <https://leetcode.com/problems/number-of-pairs-of-interchangeable-rectangles/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut counts: HashMap<i64, i64> = HashMap::new();
        let mut total = 0;

        rectangles.into_iter().for_each(|wh| {
            let e = counts
                .entry((wh[0] as i64 * 10000000000) / wh[1] as i64)
                .or_default();
            total += *e;
            *e += 1;
        });

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::interchangeable_rectangles(vec![
                vec![4, 8],
                vec![3, 6],
                vec![10, 20],
                vec![15, 30]
            ]),
            6
        );
        assert_eq!(
            Solution::interchangeable_rectangles(vec![vec![4, 5], vec![7, 8]]),
            0
        );
    }
}
