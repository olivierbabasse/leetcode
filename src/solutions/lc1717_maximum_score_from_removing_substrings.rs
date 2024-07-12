//! <https://leetcode.com/problems/maximum-score-from-removing-substrings/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn filter(s: &[u8], f: [u8; 2]) -> Vec<u8> {
        let mut stack = Vec::<u8>::new();
        for b in s {
            if let Some(b2) = stack.last() {
                if *b == f[1] && *b2 == f[0] {
                    stack.pop();
                    continue;
                }
            }
            stack.push(*b);
        }
        stack
    }

    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let (s1, s2) = if x >= y { ("ab", "ba") } else { ("ba", "ab") };
        let step1 = Self::filter(s.as_bytes(), [s1.as_bytes()[0], s1.as_bytes()[1]]);
        let step2 = Self::filter(&step1, [s2.as_bytes()[0], s2.as_bytes()[1]]);
        ((s.len() - step1.len()) as i32 * x.max(y) + (step1.len() - step2.len()) as i32 * x.min(y))
            / 2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::maximum_gain("cdbcbbaaabab".into(), 4, 5), 19);
        assert_eq!(Solution::maximum_gain("aabbaaxybbaabb".into(), 5, 4), 20);
    }
}
