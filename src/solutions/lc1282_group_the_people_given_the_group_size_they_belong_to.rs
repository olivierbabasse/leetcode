//! <https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut buckets: HashMap<_, Vec<_>> = HashMap::new();
        group_sizes
            .into_iter()
            .enumerate()
            .for_each(|(index, group_size)| {
                buckets
                    .entry(group_size)
                    .and_modify(|v| v.push(index))
                    .or_insert(vec![index]);
            });

        buckets
            .into_iter()
            .map(|(key, value)| {
                let ch = value
                    .chunks(key as usize)
                    .map(|v| v.iter().map(|v| *v as i32).collect::<Vec<_>>())
                    .collect::<Vec<_>>();

                ch
            })
            .collect::<Vec<_>>()
            .concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::array_eq;

    #[test]
    fn test_cases() {
        assert!(array_eq(
            &Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
            &[vec![5], vec![0, 1, 2], vec![3, 4, 6]]
        ));
        assert!(array_eq(
            &Solution::group_the_people(vec![2, 1, 3, 3, 3, 2]),
            &[vec![1], vec![0, 5], vec![2, 3, 4]]
        ));
    }
}
