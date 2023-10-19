//! <https://leetcode.com/problems/backspace-string-compare/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    fn fix_string(s: &mut String) {
        let mut read_pos = 0;
        let mut write_pos: usize = 0;
        while read_pos < s.len() {
            let b = s.as_bytes()[read_pos];
            if b == b'#' {
                write_pos = write_pos.saturating_sub(1);
            } else {
                unsafe {
                    s.as_bytes_mut()[write_pos] = b;
                }
                write_pos += 1;
            }
            read_pos += 1;
        }
        s.truncate(write_pos);
    }

    pub fn backspace_compare(mut s: String, mut t: String) -> bool {
        Self::fix_string(&mut s);
        Self::fix_string(&mut t);
        s == t
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert!(Solution::backspace_compare("ab#c".into(), "ad#c".into()));
        assert!(Solution::backspace_compare("ab##".into(), "c#d#".into()));
        assert!(!Solution::backspace_compare("a#c".into(), "b".into()));
        assert!(Solution::backspace_compare(
            "y#fo##f".into(),
            "y#f#o##f".into()
        ));
    }
}
