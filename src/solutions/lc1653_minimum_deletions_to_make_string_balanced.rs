//! <https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let len = s.len();
        let mut counts = vec![0; len];
        let (mut a_count, mut b_count) = (0, 0);

        for i in 0..len {
            counts[len - i - 1] += a_count;
            if s.as_bytes()[len - i - 1] == b'a' {
                a_count += 1;
            }
            counts[i] += b_count;
            if s.as_bytes()[i] == b'b' {
                b_count += 1;
            }
        }

        counts.into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::minimum_deletions("aababbab".into()), 2);
        assert_eq!(Solution::minimum_deletions("bbaaaaabb".into()), 2);
    }
}
