//! <https://leetcode.com/problems/three-consecutive-odds/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut counter = 0;
        for a in arr {
            if a % 2 != 0 {
                counter += 1;
            } else {
                counter = 0;
            }
            if counter == 3 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(!Solution::three_consecutive_odds(vec![2, 6, 4, 1]));
        assert!(Solution::three_consecutive_odds(vec![
            1, 2, 34, 3, 4, 5, 7, 23, 12
        ]));
    }
}
