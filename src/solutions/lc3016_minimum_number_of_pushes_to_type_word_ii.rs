//! <https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut counts = [0; 26];
        for b in word.as_bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        counts.sort_unstable_by(|a, b| b.cmp(a));

        let (mut step, mut index) = (1, 0);
        counts.into_iter().filter(|&c| c > 0).fold(0, |acc, c| {
            let x = c * step;
            index += 1;
            if index == 8 {
                index = 0;
                step += 1;
            }
            acc + x
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::minimum_pushes("abcde".into()), 5);
        assert_eq!(Solution::minimum_pushes("xyzxyzxyzxyz".into()), 12);
        assert_eq!(
            Solution::minimum_pushes("aabbccddeeffgghhiiiiii".into()),
            24
        );
    }
}
