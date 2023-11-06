//! <https://leetcode.com/problems/queries-on-a-permutation-with-key/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n)
impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut p = (1..=m).collect::<Vec<_>>();
        let mut positions = Vec::new();
        for query in queries {
            let pos = p.iter().position(|p| *p == query).unwrap();
            positions.push(pos as i32);
            p[0..=pos].rotate_right(1);
        }
        positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::process_queries(vec![3, 1, 2, 1], 5),
            vec![2, 1, 2, 1]
        );
        assert_eq!(
            Solution::process_queries(vec![4, 1, 2, 2], 4),
            vec![3, 1, 2, 0]
        );
        assert_eq!(
            Solution::process_queries(vec![7, 5, 5, 8, 3], 8),
            vec![6, 5, 0, 7, 5]
        );
    }
}
