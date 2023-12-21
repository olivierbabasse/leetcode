//! <https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut x = points.into_iter().map(|p| p[0]).collect::<Vec<_>>();
        x.sort_unstable();
        x.windows(2).map(|w| w[1] - w[0]).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![8, 7],
                vec![9, 9],
                vec![7, 4],
                vec![9, 7]
            ]),
            1
        );
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![3, 1],
                vec![9, 0],
                vec![1, 0],
                vec![1, 4],
                vec![5, 3],
                vec![8, 8]
            ]),
            3
        );
    }
}
