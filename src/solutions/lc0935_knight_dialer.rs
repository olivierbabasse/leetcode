//! <https://leetcode.com/problems/knight-dialer/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn calc_dials(
        count: usize,
        digit: usize,
        transitions: &Vec<Vec<usize>>,
        dp: &mut [i32],
    ) -> i32 {
        if count == 0 {
            return 1;
        }

        let cache = dp[count * 10 + digit];
        if cache > 0 {
            return cache;
        }

        let mut total = 0;
        for t in &transitions[digit] {
            total = (total + Self::calc_dials(count - 1, *t, transitions, dp)) % 1000000007;
        }

        dp[count * 10 + digit] = total;
        total
    }

    pub fn knight_dialer(n: i32) -> i32 {
        let transitions: Vec<Vec<usize>> = vec![
            vec![6, 4],
            vec![6, 8],
            vec![7, 9],
            vec![4, 8],
            vec![3, 9, 0],
            vec![],
            vec![1, 7, 0],
            vec![2, 6],
            vec![1, 3],
            vec![2, 4],
        ];

        let mut dp = vec![0; 50000];
        let mut total = 0;
        for digit in 0..10usize {
            total = (total + Self::calc_dials(n as usize - 1, digit, &transitions, &mut dp))
                % 1000000007;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::knight_dialer(1), 10);
        assert_eq!(Solution::knight_dialer(2), 20);
        assert_eq!(Solution::knight_dialer(3131), 136006598);
    }
}
