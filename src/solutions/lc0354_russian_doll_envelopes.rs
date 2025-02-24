// <https://leetcode.com/problems/russian-doll-envelopes/>

use std::cmp::Reverse;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_unstable_by_key(|e| (e[0], Reverse(e[1])));

        let heights = envelopes.into_iter().map(|e| e[1]).collect::<Vec<_>>();

        let mut lis = Vec::new();
        for h in heights {
            let p = lis.partition_point(|&p| p < h);
            if p == lis.len() {
                lis.push(h)
            } else {
                lis[p] = h;
            }
        }

        lis.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
            3
        );
        assert_eq!(
            Solution::max_envelopes(vec![vec![1, 1], vec![1, 1], vec![1, 1],]),
            1
        );
        assert_eq!(
            Solution::max_envelopes(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![3, 5],
                vec![4, 5],
                vec![5, 5],
                vec![5, 6],
                vec![6, 7],
                vec![7, 8]
            ]),
            7
        );
        assert_eq!(
            Solution::max_envelopes(vec![
                vec![1, 15],
                vec![7, 18],
                vec![7, 6],
                vec![7, 100],
                vec![2, 200],
                vec![17, 30],
                vec![17, 45],
                vec![3, 5],
                vec![7, 8],
                vec![3, 6],
                vec![3, 10],
                vec![7, 20],
                vec![17, 3],
                vec![17, 45]
            ]),
            3
        );
    }
}
