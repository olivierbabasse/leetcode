//! <https://leetcode.com/problems/stone-game-ii/>

struct Solution {}

/// time-complexity : O(n^3)
/// space-complexity : O(n^2)
impl Solution {
    fn rec(
        ssum: &[i32],
        max_stones: usize,
        index: usize,
        cache: &mut Vec<Vec<Option<i32>>>,
    ) -> i32 {
        if index + 2 * max_stones >= ssum.len() {
            return ssum[index];
        }
        if let Some(val) = cache[index][max_stones] {
            return val;
        }

        let mut res = i32::MAX;
        for i in 1..=2 * max_stones {
            res = res.min(Self::rec(ssum, i.max(max_stones), index + i, cache));
        }
        cache[index][max_stones] = Some(ssum[index] - res);
        ssum[index] - res
    }

    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let len = piles.len();
        let mut ssum = piles.clone();
        for i in (0..len - 1).rev() {
            ssum[i] += ssum[i + 1];
        }
        Self::rec(&ssum, 1, 0, &mut vec![vec![None; len]; len])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
        assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
    }
}
