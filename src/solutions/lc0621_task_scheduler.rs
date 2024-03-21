//! <https://leetcode.com/problems/task-scheduler/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freqs = [0; 26];
        for &t in &tasks {
            freqs[t as usize - 'A' as usize] += 1;
        }
        freqs.sort_unstable();
        let maxfreq = freqs[25] - 1;
        let mut cooling = maxfreq * n;
        let mut i = 24i32;
        while i >= 0 && freqs[i as usize] > 0 {
            cooling -= maxfreq.min(freqs[i as usize]);
            i -= 1;
        }
        tasks.len() as i32 + cooling.max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
            8
        );
        assert_eq!(
            Solution::least_interval(vec!['A', 'C', 'A', 'B', 'D', 'B'], 1),
            6
        );
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 3),
            10
        );
    }
}
