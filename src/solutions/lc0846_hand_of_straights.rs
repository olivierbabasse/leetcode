//! <https://leetcode.com/problems/hand-of-straights/>

use std::collections::BTreeMap;

struct Solution {}

/// time-complexity : O(n * log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }

        let mut freqs = BTreeMap::<i32, i32>::new();
        hand.into_iter()
            .for_each(|v| *freqs.entry(v).or_default() += 1);

        while !freqs.is_empty() {
            let cur = *freqs.first_key_value().unwrap().0;
            for i in 0..group_size {
                if !freqs.contains_key(&(cur + i)) {
                    return false;
                }
                *freqs.get_mut(&(cur + i)).unwrap() -= 1;
                if *freqs.get(&(cur + i)).unwrap() == 0 {
                    freqs.remove(&(cur + i));
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::is_n_straight_hand(
            vec![1, 2, 3, 6, 2, 3, 4, 7, 8],
            3
        ));
        assert!(!Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4));
    }
}
