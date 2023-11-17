//! <https://leetcode.com/problems/reordered-power-of-2/>

struct Solution {}

/// time-complexity : O(log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let digit_counts = |mut n: i32| -> [usize; 10] {
            let mut counts = [0; 10];
            while n > 0 {
                counts[(n % 10) as usize] += 1;
                n /= 10;
            }
            counts
        };

        (0..30).any(|p| digit_counts(1 << p) == digit_counts(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::reordered_power_of2(1));
        assert!(!Solution::reordered_power_of2(10));
    }
}
