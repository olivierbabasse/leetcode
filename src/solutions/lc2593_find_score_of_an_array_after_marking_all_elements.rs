//! <https://leetcode.com/problems/find-score-of-an-array-after-marking-all-elements/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut nums = nums
            .into_iter()
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect::<Vec<_>>();
        nums.sort_unstable();
        let mut marked = vec![false; nums.len()];
        let mut res = 0;
        for (num, index) in nums.into_iter() {
            if !marked[index] {
                res += num as i64;
                for j in index.saturating_sub(1)..=(index + 1).min(marked.len() - 1) {
                    marked[j] = true;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_score(vec![2, 1, 3, 4, 5, 2]), 7);
        assert_eq!(Solution::find_score(vec![2, 3, 5, 1, 3, 2]), 5);
    }
}
