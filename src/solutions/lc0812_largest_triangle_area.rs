//! <https://leetcode.com/problems/largest-triangle-area/>

struct Solution {}

/// time-complexity : O(n^3)
/// space-complexity : O(1)
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut res = 0f64;
        for (i, p0) in points.iter().enumerate() {
            for (j, p1) in points.iter().skip(i).enumerate() {
                for p2 in points.iter().skip(j) {
                    let (x1, y1) = (p0[0] as f64, p0[1] as f64);
                    let (x2, y2) = (p1[0] as f64, p1[1] as f64);
                    let (x3, y3) = (p2[0] as f64, p2[1] as f64);
                    res = res.max((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs() / 2.0);
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
        assert_eq!(
            Solution::largest_triangle_area(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![0, 2],
                vec![2, 0]
            ]),
            2.0
        );
        assert_eq!(
            Solution::largest_triangle_area(vec![vec![1, 0], vec![0, 0], vec![0, 1]]),
            0.5
        );
    }
}
