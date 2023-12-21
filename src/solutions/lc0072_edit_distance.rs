//! <https://leetcode.com/problems/edit-distance/>

struct Solution {}

/// time-complexity : O(len(word1)*len(word2))
/// space-complexity : O(len(word1)*len(word2))
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let len1 = word1.len();
        let len2 = word2.len();
        let mut d = vec![vec![0; len2 + 1]; len1 + 1];
        for (i, item) in d.iter_mut().enumerate() {
            item[0] = i;
        }
        for j in 0..=len2 {
            d[0][j] = j;
        }
        for i in 1..=len1 {
            for j in 1..=len2 {
                d[i][j] = (d[i - 1][j] + 1).min(d[i][j - 1] + 1).min(
                    d[i - 1][j - 1]
                        + if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] {
                            0
                        } else {
                            1
                        },
                );
            }
        }
        d[len1][len2] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_distance("horse".into(), "ros".into()), 3);
        assert_eq!(
            Solution::min_distance("intention".into(), "execution".into()),
            5
        );
    }
}
