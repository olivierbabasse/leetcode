//! <https://leetcode.com/problems/maximum-length-of-pair-chain/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by_key(|p| p[1]);
        let mut last = i32::MIN;
        let mut count = 0;
        for p in pairs {
            if p[0] > last {
                count += 1;
                last = p[1];
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
            2
        );
        assert_eq!(
            Solution::find_longest_chain(vec![vec![1, 2], vec![7, 8], vec![4, 5]]),
            3
        );
    }
}
