//! <https://leetcode.com/problems/k-inverse-pairs-array/>

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let (n, k) = (n as usize, k as usize);
        let mut dp = vec![vec![0; k + 1]; n + 1];
        dp[0][0] = 1;

        for i in 1..=n {
            let mut prefix_sum = vec![0; k + 1];
            prefix_sum[0] = dp[i - 1][0];

            for j in 1..=k {
                prefix_sum[j] = (prefix_sum[j - 1] + dp[i - 1][j]) % 1000000007;
            }

            for j in 0..=k {
                dp[i][j] =
                    (prefix_sum[j] - if j >= i { prefix_sum[j - i] } else { 0 }) % 1000000007;
            }
        }

        (dp[n][k] + 1000000007) % 1000000007
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::k_inverse_pairs(3, 0), 1);
        assert_eq!(Solution::k_inverse_pairs(3, 1), 2);
        assert_eq!(Solution::k_inverse_pairs(10, 11), 32683);
        assert_eq!(Solution::k_inverse_pairs(1000, 1000), 663677020);
    }
}
