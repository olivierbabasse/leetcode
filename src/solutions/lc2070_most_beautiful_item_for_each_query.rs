//! <https://leetcode.com/problems/most-beautiful-item-for-each-query/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable();
        items.dedup_by(|a, b| a[1] <= b[1]);
        queries
            .into_iter()
            .map(|q| match items.partition_point(|item| q >= item[0]) {
                0 => 0,
                e => items[e - 1][1],
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::maximum_beauty(
                vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
                vec![1, 2, 3, 4, 5, 6]
            ),
            vec![2, 4, 5, 5, 6, 6]
        );
        assert_eq!(
            Solution::maximum_beauty(
                vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]],
                vec![1]
            ),
            vec![4]
        );
        assert_eq!(
            Solution::maximum_beauty(vec![vec![10, 1000]], vec![5]),
            vec![0]
        );
    }
}
