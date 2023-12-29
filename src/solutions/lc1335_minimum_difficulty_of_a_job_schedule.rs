//! <https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/>

struct Solution {}

/// time-complexity : O(days*len)
/// space-complexity : O(days*len)
impl Solution {
    fn rec(
        job_index: usize,
        days_left: usize,
        job_diff: &Vec<i32>,
        cache: &mut Vec<Vec<Option<i32>>>,
    ) -> i32 {
        let len = job_diff.len();
        if job_index == len && days_left == 0 {
            return 0;
        }
        if job_index == len || days_left == 0 {
            return i32::MAX;
        }

        if let Some(val) = cache[days_left][job_index] {
            return val;
        }

        let mut res = i32::MAX;
        let mut day_diff = 0;
        for j in job_index..=(len - days_left) {
            day_diff = day_diff.max(job_diff[j]);
            res = res.min(i32::saturating_add(
                day_diff,
                Self::rec(j + 1, days_left - 1, job_diff, cache),
            ))
        }
        cache[days_left][job_index] = Some(res);
        res
    }

    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        if d as usize > job_difficulty.len() {
            return -1;
        }

        let mut cache = vec![vec![None; job_difficulty.len() + 1]; d as usize + 1];
        let res = Self::rec(0, d as usize, &job_difficulty, &mut cache);
        if res == i32::MAX {
            -1
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_difficulty(vec![6, 5, 4, 3, 2, 1], 2), 7);
        assert_eq!(Solution::min_difficulty(vec![9, 9, 9], 4), -1);
        assert_eq!(Solution::min_difficulty(vec![1, 1, 1], 3), 3);
        assert_eq!(
            Solution::min_difficulty(
                vec![
                    380, 302, 102, 681, 863, 676, 243, 671, 651, 612, 162, 561, 394, 856, 601, 30,
                    6, 257, 921, 405, 716, 126, 158, 476, 889, 699, 668, 930, 139, 164, 641, 801,
                    480, 756, 797, 915, 275, 709, 161, 358, 461, 938, 914, 557, 121, 964, 315
                ],
                10
            ),
            3807
        );
    }
}
