//! <https://leetcode.com/problems/find-champion-ii/>

struct Solution {}

/// time-complexity : O(n+len(edges))
/// space-complexity : O(n)
impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut indegree = vec![0; n as usize];
        for e in edges {
            indegree[e[1] as usize] += 1;
        }

        let champions = indegree
            .iter()
            .enumerate()
            .filter(|(_, &count)| count == 0)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if champions.len() == 1 {
            champions[0] as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_champion(3, vec![vec![0, 1], vec![1, 2]]), 0);
        assert_eq!(
            Solution::find_champion(4, vec![vec![0, 2], vec![1, 3], vec![1, 2]]),
            -1
        );
    }
}
