//! <https://leetcode.com/problems/crawler-log-folder/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut counter = 0;
        for l in logs {
            if l.starts_with("../") {
                counter = 0.max(counter - 1);
            } else if l.starts_with("./") {
            } else {
                counter += 1;
            }
        }
        counter
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".into(),
                "d2/".into(),
                "../".into(),
                "d21/".into(),
                "./".into()
            ]),
            2
        );
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".into(),
                "d2/".into(),
                "./".into(),
                "d3/".into(),
                "../".into(),
                "d31/".into()
            ]),
            3
        );
        assert_eq!(
            Solution::min_operations(vec!["d1/".into(), "../".into(), "../".into(), "../".into()]),
            0
        );
    }
}
