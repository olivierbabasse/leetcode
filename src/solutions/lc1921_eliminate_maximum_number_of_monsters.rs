//! <https://leetcode.com/problems/eliminate-maximum-number-of-monsters/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut arrival = dist
            .into_iter()
            .zip(speed)
            .map(|(d, s)| (d as i64) * 100000 / (s as i64))
            .collect::<Vec<_>>();
        arrival.sort_unstable();
        for (i, arrival) in arrival.iter().enumerate().skip(1) {
            if (100000 * i as i64) >= *arrival {
                return i as i32;
            }
        }
        arrival.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::eliminate_maximum(vec![1, 3, 4], vec![1, 1, 1]), 3);
        assert_eq!(
            Solution::eliminate_maximum(vec![1, 1, 2, 3], vec![1, 1, 1, 1]),
            1
        );
        assert_eq!(
            Solution::eliminate_maximum(vec![3, 5, 7, 4, 5], vec![2, 3, 6, 3, 2]),
            2
        );
        assert_eq!(
            Solution::eliminate_maximum(vec![100000, 1], vec![99999, 1]),
            2
        );
    }
}
