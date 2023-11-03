//! <https://leetcode.com/problems/build-an-array-with-stack-operations/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
        let mut i = 1;
        let mut output = Vec::new();
        for val in target {
            while i < val {
                output.push("Push");
                output.push("Pop");
                i += 1;
            }
            output.push("Push");
            i += 1;
        }
        output.into_iter().map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            vec![
                "Push".to_string(),
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string()
            ]
        );
        assert_eq!(
            Solution::build_array(vec![1, 2, 3], 3),
            vec!["Push".to_string(), "Push".to_string(), "Push".to_string()]
        );
        assert_eq!(
            Solution::build_array(vec![1, 2], 4),
            vec!["Push".to_string(), "Push".to_string(),]
        );
    }
}
