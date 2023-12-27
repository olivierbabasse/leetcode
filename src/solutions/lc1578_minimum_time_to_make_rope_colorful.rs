//! <https://leetcode.com/problems/minimum-time-to-make-rope-colorful/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let colors = colors.as_bytes();
        let (mut res, mut prev, mut max, mut sum) = (0, 0, 0, 0);

        for (&c, &nt) in colors.iter().zip(needed_time.iter()) {
            if c != prev {
                prev = c;
                res += sum - max;
                max = nt;
                sum = 0;
            } else {
                max = max.max(nt);
            }
            sum += nt;
        }

        res + sum - max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_cost("abaac".into(), vec![1, 2, 3, 4, 5]), 3);
        assert_eq!(Solution::min_cost("abc".into(), vec![1, 2, 3]), 0);
        assert_eq!(Solution::min_cost("aabaa".into(), vec![1, 2, 3, 4, 1]), 2);
    }
}
