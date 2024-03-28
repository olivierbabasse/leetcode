//! <https://leetcode.com/problems/optimal-partition-of-string/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut chars = [false; 26];
        let mut count = 1;
        for b in s.bytes() {
            if chars[(b - b'a') as usize] {
                chars.fill(false);
                count += 1;
            }
            chars[(b - b'a') as usize] = true;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::partition_string("abacaba".into()), 4);
        assert_eq!(Solution::partition_string("ssssss".into()), 6);
    }
}
