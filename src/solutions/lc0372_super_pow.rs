//! <https://leetcode.com/problems/super-pow/>

struct Solution {}

/// time-complexity : O(len(b))
/// space-complexity : O(1)
impl Solution {
    fn pow_mod(mut a: i32, k: i32) -> i32 {
        let mut res = 1;
        a %= 1337;
        for _ in 0..k {
            res = (res * a) % 1337;
        }
        res
    }

    pub fn super_pow(a: i32, mut b: Vec<i32>) -> i32 {
        if let Some(digit) = b.pop() {
            Self::pow_mod(Self::super_pow(a, b), 10) * Self::pow_mod(a, digit) % 1337
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::super_pow(2, vec![3]), 8);
        assert_eq!(Solution::super_pow(2, vec![1, 0]), 1024);
        assert_eq!(Solution::super_pow(1, vec![4, 3, 3, 8, 5, 2]), 1);
    }
}
