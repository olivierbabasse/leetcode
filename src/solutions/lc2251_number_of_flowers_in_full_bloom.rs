//! <https://leetcode.com/problems/number-of-flowers-in-full-bloom/>

struct Solution {}

impl Solution {
    /// time-complexity : O(n.log(n))
    /// space-complexity : O(n)
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut start = flowers.iter().map(|f| f[0]).collect::<Vec<_>>();
        start.sort_unstable();
        let mut stop = flowers.iter().map(|f| f[1]).collect::<Vec<_>>();
        stop.sort_unstable();
        people
            .into_iter()
            .map(|p| (start.partition_point(|&t| t <= p) - stop.partition_point(|&t| t < p)) as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::full_bloom_flowers(
                vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]],
                vec![2, 3, 7, 11]
            ),
            vec![1, 2, 2, 2]
        );
        assert_eq!(
            Solution::full_bloom_flowers(vec![vec![1, 10], vec![3, 3]], vec![3, 3, 2]),
            vec![2, 2, 1]
        );
    }
}
