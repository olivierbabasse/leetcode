//! <https://leetcode.com/problems/bulb-switcher-ii/>

use std::collections::HashSet;

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        let presses = presses as u32;
        let mut set = HashSet::new();
        let n = 6.min(n);
        let shift = 0.max(6 - n);
        for buttons in 0..16u32 {
            let bitcount = buttons.count_ones();
            if bitcount % 2 == presses % 2 && bitcount <= presses {
                let mut lights = 0u32;
                if (buttons & 1) != 0 {
                    lights ^= 0b111111 >> shift;
                }
                if ((buttons >> 1) & 1) != 0 {
                    lights ^= 0b010101 >> shift;
                }
                if ((buttons >> 2) & 1) != 0 {
                    lights ^= 0b101010 >> shift;
                }
                if ((buttons >> 3) & 1) != 0 {
                    lights ^= 0b100100 >> shift;
                }
                set.insert(lights);
            }
        }
        set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::flip_lights(1, 1), 2);
        assert_eq!(Solution::flip_lights(2, 1), 3);
        assert_eq!(Solution::flip_lights(3, 1), 4);
    }
}
