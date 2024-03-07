//! <https://leetcode.com/problems/bag-of-tokens/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        let mut score = 0;
        if !tokens.is_empty() {
            tokens.sort_unstable();
            let (mut i, mut j) = (0, tokens.len() - 1);
            while i <= j {
                if tokens[i] <= power {
                    power -= tokens[i];
                    i += 1;
                    score += 1;
                } else if i < j && score > 0 {
                    power += tokens[j];
                    j -= 1;
                    score -= 1;
                } else {
                    break;
                }
            }
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100], 50), 0);
        assert_eq!(Solution::bag_of_tokens_score(vec![200, 100], 150), 1);
        assert_eq!(
            Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200),
            2
        );
        assert_eq!(Solution::bag_of_tokens_score(vec![], 85), 0);
    }
}
