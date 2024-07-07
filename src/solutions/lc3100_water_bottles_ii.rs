//! <https://leetcode.com/problems/water-bottles-ii/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut count = num_bottles;
        let mut cur = num_bottles;
        while num_exchange <= cur {
            cur += 1 - num_exchange;
            num_exchange += 1;
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_bottles_drunk(13, 6), 15);
        assert_eq!(Solution::max_bottles_drunk(10, 3), 13);
    }
}
