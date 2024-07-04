//! <https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable();
        let mut last = points[0].clone();
        let mut count = 1;
        for p in points.into_iter().skip(1) {
            if last[1] >= p[0] {
                last[0] = last[0].max(p[0]);
                last[1] = last[1].min(p[1]);
            } else {
                count += 1;
                last.clone_from(&p);
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
            2
        );
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
            4
        );
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
            2
        );
    }
}
