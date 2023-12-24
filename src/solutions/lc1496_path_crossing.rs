//! <https://leetcode.com/problems/path-crossing/>

use std::collections::HashSet;

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut positions = HashSet::new();
        let (mut x, mut y) = (0, 0);
        positions.insert((0, 0));

        for c in path.chars() {
            match c {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => {}
            }

            if positions.contains(&(x, y)) {
                return true;
            }
            positions.insert((x, y));
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(!Solution::is_path_crossing("NES".into()));
        assert!(Solution::is_path_crossing("NESWW".into()));
    }
}
