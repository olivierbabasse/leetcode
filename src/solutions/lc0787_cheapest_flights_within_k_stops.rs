//! <https://leetcode.com/problems/cheapest-flights-within-k-stops/>

use std::collections::{HashMap, VecDeque};

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn find_cheapest_price(
        n: i32,
        flights: Vec<Vec<i32>>,
        src: i32,
        dst: i32,
        mut k: i32,
    ) -> i32 {
        let mut adj: HashMap<i32, Vec<_>> = HashMap::new();
        for f in flights {
            adj.entry(f[0]).or_default().push((f[1], f[2]));
        }
        let mut prices = vec![None; n as usize];

        let mut queue = VecDeque::new();
        queue.push_back((src, 0));

        while k >= 0 {
            let n = queue.len();
            for _ in 0..n {
                let (cur_dest, cur_price) = queue.pop_front().unwrap();
                for (next_dest, next_price) in adj.get(&cur_dest).unwrap_or(&Vec::new()) {
                    let new_price = cur_price + next_price;
                    if new_price < prices.get(*next_dest as usize).unwrap().unwrap_or(i32::MAX) {
                        prices[*next_dest as usize] = Some(new_price);
                        queue.push_back((*next_dest, new_price));
                    }
                }
            }

            k -= 1;
        }

        prices[dst as usize].unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_cheapest_price(
                4,
                vec![
                    vec![0, 1, 100],
                    vec![1, 2, 100],
                    vec![2, 0, 100],
                    vec![1, 3, 600],
                    vec![2, 3, 200]
                ],
                0,
                3,
                1
            ),
            700
        );
        assert_eq!(
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                1
            ),
            200
        );
        assert_eq!(
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                0
            ),
            500
        );
    }
}
