//! <https://leetcode.com/problems/water-bottles/>

struct Solution {}

/// time-complexity : O(log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut count = 0;
        while num_bottles >= num_exchange {
            let q = num_bottles / num_exchange;
            count += q * num_exchange;
            num_bottles += q * (1 - num_exchange);
        }
        count + num_bottles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }
}
