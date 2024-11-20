//! <https://leetcode.com/problems/take-k-of-each-character-from-left-and-right/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut counts = [0; 3];
        for b in s {
            counts[(b - b'a') as usize] += 1;
        }
        if counts.iter().any(|&c| c < k) {
            return -1;
        }

        let mut window = [0; 3];
        let mut left = 0;
        let mut max = 0;

        for right in 0..s.len() {
            window[(s[right] - b'a') as usize] += 1;
            while left <= right && (counts.iter().zip(window).any(|(a, b)| a - b < k)) {
                window[(s[left] - b'a') as usize] -= 1;
                left += 1;
            }

            max = max.max(right as i32 - left as i32 + 1);
        }

        s.len() as i32 - max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::take_characters("aabaaaacaabc".into(), 2), 8);
        assert_eq!(Solution::take_characters("a".into(), 1), -1);
    }
}
