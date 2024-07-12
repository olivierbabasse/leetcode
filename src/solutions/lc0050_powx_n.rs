//! <https://leetcode.com/problems/powx-n/>

struct Solution {}

/// time-complexity : O(log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        let mut n = n as i64;
        if n < 0 {
            x = 1.0 / x;
            n = -n;
        }
        let mut res = 1.0;
        while n > 0 {
            if n % 2 != 0 {
                res *= x;
            }
            x *= x;
            n /= 2;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
        assert!((Solution::my_pow(2.1, 3) - 9.261).abs() < 0.00001);
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);
        assert_eq!(Solution::my_pow(2.0, -2147483648), 0.0);
    }
}
