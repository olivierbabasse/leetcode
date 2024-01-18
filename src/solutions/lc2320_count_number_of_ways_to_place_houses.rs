//! <https://leetcode.com/problems/count-number-of-ways-to-place-houses/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        let (mut a, mut b) = (2i64, 1i64);
        for _ in 0..n - 1 {
            (a, b) = ((a + b) % 1000000007, a);
        }
        ((a * a) % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_house_placements(1), 4);
        assert_eq!(Solution::count_house_placements(2), 9);
        assert_eq!(Solution::count_house_placements(1000), 500478595);
    }
}
