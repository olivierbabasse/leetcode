//! <https://leetcode.com/problems/counting-bits/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(n)
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![0];
        for i in 1..=n {
            let val = res[(i / 2) as usize];
            if i % 2 == 0 {
                res.push(val);
            } else {
                res.push(val + 1);
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
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
