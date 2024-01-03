//! <https://leetcode.com/problems/find-lucky-integer-in-an-array/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut freqs = vec![0; 501];
        arr.into_iter().for_each(|a| freqs[a as usize] += 1);
        freqs
            .into_iter()
            .enumerate()
            .rev()
            .filter(|(index, a)| a == index && a > &0)
            .max()
            .map(|(_, a)| a as i32)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
        assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
        assert_eq!(Solution::find_lucky(vec![2, 2, 2, 3, 3]), -1);
    }
}
