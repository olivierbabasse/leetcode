//! <https://leetcode.com/problems/determine-if-two-events-have-conflict/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    fn parse_time(t: &str) -> i32 {
        let parts = t.split(':').collect::<Vec<_>>();
        parts[0].parse::<i32>().unwrap() * 60 + parts[1].parse::<i32>().unwrap()
    }

    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let (s1, e1) = (Self::parse_time(&event1[0]), Self::parse_time(&event1[1]));
        let (s2, e2) = (Self::parse_time(&event2[0]), Self::parse_time(&event2[1]));
        if s1 <= s2 {
            e1 >= s2
        } else {
            e2 >= s1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::have_conflict(
            vec!["01:15".into(), "02:00".into()],
            vec!["02:00".into(), "03:00".into()]
        ));
        assert!(Solution::have_conflict(
            vec!["01:00".into(), "02:00".into()],
            vec!["01:20".into(), "03:00".into()]
        ));
        assert!(!Solution::have_conflict(
            vec!["10:00".into(), "11:00".into()],
            vec!["14:00".into(), "15:00".into()]
        ));
    }
}
