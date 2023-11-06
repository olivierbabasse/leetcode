//! <https://leetcode.com/problems/find-the-winner-of-an-array-game/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut winner = arr[0];
        let mut winner_count = -1;
        for num in arr.into_iter() {
            if num > winner {
                winner = num;
                winner_count = 1;
            } else {
                winner_count += 1;
            }
            if winner_count == k {
                break;
            }
        }
        winner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2), 5);
        assert_eq!(Solution::get_winner(vec![3, 2, 1], 10), 3);
    }
}
