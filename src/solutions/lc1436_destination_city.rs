//! <https://leetcode.com/problems/destination-city/>

use std::collections::HashSet;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let starts = paths
            .iter()
            .map(|path| path[0].clone())
            .collect::<HashSet<_>>();
        paths
            .into_iter()
            .find(|path| !starts.contains(&path[1]))
            .map(|path| path[1].clone())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::dest_city(vec![
                vec!["B".into(), "C".into()],
                vec!["D".into(), "B".into()],
                vec!["C".into(), "A".into()]
            ]),
            "A".to_string()
        )
    }
}
