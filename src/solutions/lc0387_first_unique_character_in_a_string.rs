//! <https://leetcode.com/problems/first-unique-character-in-a-string/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut freqs = vec![0; 256];
        s.as_bytes().iter().for_each(|&b| freqs[b as usize] += 1);
        for (i, &b) in s.as_bytes().iter().enumerate() {
            if freqs[b as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::first_uniq_char("leetcode".into()), 0);
        assert_eq!(Solution::first_uniq_char("loveleetcode".into()), 2);
        assert_eq!(Solution::first_uniq_char("aabb".into()), -1);
    }
}
