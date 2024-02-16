//! <https://leetcode.com/problems/split-with-minimum-sum/>

struct Solution {}

/// time-complexity : O(len(n))
/// space-complexity : O(len(n))
impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let mut digits = format!("{num}").chars().collect::<Vec<_>>();
        digits.sort_unstable();
        let mut output1 = Vec::new();
        let mut output2 = Vec::new();
        for (i, d) in digits.into_iter().enumerate() {
            if i % 2 == 0 {
                output1.push(d);
            } else {
                output2.push(d);
            }
        }
        let n1 = output1.into_iter().collect::<String>();
        let n2 = output2.into_iter().collect::<String>();
        n1.parse::<i32>().unwrap() + n2.parse::<i32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::split_num(4325), 59);
        assert_eq!(Solution::split_num(687), 75);
    }
}
