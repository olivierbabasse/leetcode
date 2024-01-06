//! <https://leetcode.com/problems/maximum-profit-in-job-scheduling/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    fn rec(jobs: &[(i32, i32, i32)], index: usize, cache: &mut [Option<i32>]) -> i32 {
        if index >= jobs.len() {
            0
        } else if let Some(res) = cache[index] {
            res
        } else {
            let next_index = jobs.partition_point(|j| j.0 < jobs[index].1);
            let res = Self::rec(jobs, index + 1, cache)
                .max(jobs[index].2 + Self::rec(jobs, next_index, cache));
            cache[index] = Some(res);
            res
        }
    }
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = start_time
            .into_iter()
            .zip(end_time)
            .zip(profit)
            .map(|((a, b), c)| (a, b, c))
            .collect::<Vec<_>>();
        jobs.sort_unstable();
        let mut cache = vec![None; jobs.len()];
        Self::rec(&jobs, 0, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            120
        );
        /*assert_eq!(
            Solution::job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            ),
            150
        );
        assert_eq!(
            Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
            6
        );*/
    }
}
