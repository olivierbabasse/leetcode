//! <https://leetcode.com/problems/number-of-longest-increasing-subsequence/>

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut len = vec![1; n];
        let mut count = vec![1; n];

        for i in 0..n {
            for j in 0..i {
                if nums[j] < nums[i] {
                    if len[j] + 1 > len[i] {
                        len[i] = len[j] + 1;
                        count[i] = 0;
                    }
                    if len[j] + 1 == len[i] {
                        count[i] += count[j];
                    }
                }
            }
        }

        let max = *len.iter().max().unwrap_or(&0);
        count
            .into_iter()
            .zip(len)
            .filter(|&(_, l)| l == max)
            .map(|(c, _)| c)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
        assert_eq!(Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
        assert_eq!(
            Solution::find_number_of_lis(vec![
                10, 70, 3, 73, 73, 71, 58, -32, -43, 41, 63, 53, -71, -53, 58, 15, -7, -27, 77, 10,
                9, -12, -59, 57, 24, 67, 37, 74, 83, 29, 13, 5, 85, 15, 81, 50, 28, -100, 30, 60,
                69, -93, 65, 6, -24, 44, 87, 78, 88, 46, 93, -43, 13, 23, 98, 56, -64, 5, 57, 92,
                49, 51, 29, 51, 1, 62, 71, -98, 7, 47, 22, -69, -90, 90, 8, 47, 88, 12, 25, 40, 56,
                42, -6, 18, 45, 30, 91, -64, 37, -1, 65, 84, 43, 63, 31, 74, 55, 89, 21, -89
            ]),
            186
        );
        assert_eq!(
            Solution::find_number_of_lis(vec![
                43, 36, 58, 22, 91, 15, 79, -91, 88, 17, 78, 46, -95, -53, -36, 52, -5, 70, -4, 13,
                30, 4, 70, 94, 24, -17, 8, 77, 32, 86, 37, -22, 30, 54, 69, 54, 15, 82, 45, 37, 32,
                82, 43, 47, 84, 30, 58, 74, 64, 41, 84, 16, 23, 39, 35, 69, 45, 57, 19, 84, -26,
                49, 3, -44, 42, 77, 25, 28, 5, 92, -42, 51, 76, 81, 25, 55, 87, 93, 49, 34, 80, 99
            ]),
            84
        );
    }
}
