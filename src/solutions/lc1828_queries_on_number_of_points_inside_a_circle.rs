//! <https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/>

struct Solution {}

/// time-complexity : O(p*q)
/// space-complexity : O(q)
impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();

        for q in &queries {
            let mut count = 0;
            for p in &points {
                if (q[0] - p[0]) * (q[0] - p[0]) + (q[1] - p[1]) * (q[1] - p[1]) <= q[2] * q[2] {
                    count += 1;
                }
            }
            res.push(count);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::count_points(
                vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]],
                vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]]
            ),
            vec![3, 2, 2]
        );
    }
}
