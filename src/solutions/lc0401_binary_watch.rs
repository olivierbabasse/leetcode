//! <https://leetcode.com/problems/binary-watch/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut res = vec![];
        for h in 0..12 {
            let ch = i32::count_ones(h);
            for m in 0..60 {
                if ch + i32::count_ones(m) == turned_on as u32 {
                    res.push(format!("{h}:{m:02}"));
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::read_binary_watch(1),
            vec![
                "0:01".to_string(),
                "0:02".to_string(),
                "0:04".to_string(),
                "0:08".to_string(),
                "0:16".to_string(),
                "0:32".to_string(),
                "1:00".to_string(),
                "2:00".to_string(),
                "4:00".to_string(),
                "8:00".to_string()
            ]
        );
        assert!(Solution::read_binary_watch(9).is_empty());
    }
}
