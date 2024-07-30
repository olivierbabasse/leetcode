//! <https://leetcode.com/problems/count-number-of-teams/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n)
impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let len = rating.len();
        let mut inc_teams = vec![vec![0; 4]; len];
        let mut dec_teams = vec![vec![0; 4]; len];

        for i in 0..len {
            inc_teams[i][1] = 1;
            dec_teams[i][1] = 1;
        }

        for count in 2..=3 {
            for i in 0..len {
                for j in i + 1..len {
                    if rating[j] > rating[i] {
                        inc_teams[j][count] += inc_teams[i][count - 1];
                    }
                    if rating[j] < rating[i] {
                        dec_teams[j][count] += dec_teams[i][count - 1];
                    }
                }
            }
        }

        let mut teams = 0;
        for i in 0..len {
            teams += inc_teams[i][3] + dec_teams[i][3];
        }

        teams
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_teams(vec![2, 5, 3, 4, 1]), 3);
        assert_eq!(Solution::num_teams(vec![2, 1, 3]), 0);
        assert_eq!(Solution::num_teams(vec![1, 2, 3, 4]), 4);
    }
}
