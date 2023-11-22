//! <https://leetcode.com/problems/count-ways-to-group-overlapping-ranges/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n*log(n))
impl Solution {
    pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
        ranges.sort_unstable_by_key(|v| v[0]);

        let mut curend = ranges[0][1];
        let mut distinct = 1;
        for v in ranges.iter().skip(1) {
            if v[0] > curend {
                distinct += 1;
            }
            curend = curend.max(v[1]);
        }

        let mut total = 1;
        while distinct > 0 {
            total = (total * 2) % 1000000007;
            distinct -= 1;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_ways(vec![vec![6, 10], vec![5, 15]]), 2);
        assert_eq!(
            Solution::count_ways(vec![vec![1, 3], vec![10, 20], vec![2, 5], vec![4, 8]]),
            4
        );
        assert_eq!(
            Solution::count_ways(vec![
                vec![34, 56],
                vec![28, 29],
                vec![12, 16],
                vec![11, 48],
                vec![28, 54],
                vec![22, 55],
                vec![28, 41],
                vec![41, 44]
            ]),
            2
        );
    }
}
