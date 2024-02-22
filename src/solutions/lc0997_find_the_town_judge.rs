//! <https://leetcode.com/problems/find-the-town-judge/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut t_in = vec![0; n as usize + 1];
        let mut t_out = vec![0; n as usize + 1];
        for t in trust {
            t_in[t[0] as usize] += 1;
            t_out[t[1] as usize] += 1;
        }
        t_in.into_iter()
            .enumerate()
            .skip(1)
            .find(|(c, t)| *t == 0 && t_out[*c] == n - 1)
            .map(|(c, _)| c as i32)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
        assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
        assert_eq!(
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
            -1
        );
        assert_eq!(Solution::find_judge(3, vec![vec![1, 2], vec![2, 3]]), -1);
    }
}
