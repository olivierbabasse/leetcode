//! <https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        let len = skill.len();
        skill.sort_unstable();

        let team_skill = skill[0] + skill[len - 1];
        let mut chemistry = 0i64;
        for i in 0..len / 2 {
            if team_skill != skill[i] + skill[len - i - 1] {
                return -1;
            }
            chemistry += (skill[i] * skill[len - i - 1]) as i64;
        }

        chemistry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::divide_players(vec![3, 2, 5, 1, 3, 4]), 22);
        assert_eq!(Solution::divide_players(vec![3, 4]), 12);
        assert_eq!(Solution::divide_players(vec![1, 1, 2, 3]), -1);
    }
}
