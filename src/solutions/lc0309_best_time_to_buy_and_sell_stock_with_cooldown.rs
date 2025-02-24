//! <https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/>

struct Solution {}

#[derive(Clone, Copy)]
enum State {
    Ready,
    Bought,
    OnCooldown,
}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    fn rec(prices: &[i32], index: usize, state: State, cache: &mut [Vec<Option<i32>>]) -> i32 {
        if index >= prices.len() {
            0
        } else if let Some(res) = cache[index][state as usize] {
            return res;
        } else {
            let res = match state {
                State::Ready => (Self::rec(prices, index + 1, State::Bought, cache)
                    - prices[index])
                    .max(Self::rec(prices, index + 1, State::Ready, cache)),
                State::Bought => Self::rec(prices, index + 1, State::Bought, cache)
                    .max(Self::rec(prices, index + 1, State::OnCooldown, cache) + prices[index]),
                State::OnCooldown => Self::rec(prices, index + 1, State::Ready, cache),
            };
            cache[index][state as usize] = Some(res);
            res
        }
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut cache = vec![vec![None; 3]; prices.len()];
        Self::rec(&prices, 0, State::Ready, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(Solution::max_profit(vec![1]), 0);
    }
}
