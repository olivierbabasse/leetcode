//! <https://leetcode.com/problems/remove-colored-pieces-if-both-neighbors-are-the-same-color/>

struct Solution {}

impl Solution {
    /// time-complexity : O(n^2)
    /// space-complexity : O(1)
    pub fn winner_of_game(colors: String) -> bool {
        let mut astart = 0;
        let mut bstart = 0;

        loop {
            if let Some(pos) = colors[astart..].find("AAA") {
                astart += pos + 1;
            } else {
                return false;
            }

            if let Some(pos) = colors[bstart..].find("BBB") {
                bstart += pos + 1;
            } else {
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert!(Solution::winner_of_game("AAABABB".into()));
        assert!(!Solution::winner_of_game("AA".into()));
        assert!(!Solution::winner_of_game("ABBBBBBBAAA".into()));
    }
}
