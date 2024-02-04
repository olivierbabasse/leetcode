//! <https://leetcode.com/problems/minimum-window-substring/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut char_freqs = vec![0; 256];
        let mut missing_chars = t.len();

        t.bytes().for_each(|b| char_freqs[b as usize] += 1);

        let (mut begin, mut end) = (0, 0);
        let mut window = None;
        while end < s.len() {
            if char_freqs[s.as_bytes()[end] as usize] > 0 {
                missing_chars -= 1;
            }

            char_freqs[s.as_bytes()[end] as usize] -= 1;
            end += 1;

            while missing_chars == 0 {
                if let Some((b, e)) = window {
                    if end - begin < e - b {
                        window = Some((begin, end));
                    }
                } else {
                    window = Some((begin, end));
                }

                char_freqs[s.as_bytes()[begin] as usize] += 1;

                if char_freqs[s.as_bytes()[begin] as usize] > 0 {
                    missing_chars += 1;
                }

                begin += 1;
            }
        }

        if let Some((b, e)) = window {
            s[b..e].into()
        } else {
            "".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".into(), "ABC".into()),
            "BANC".to_string()
        );
        assert_eq!(
            Solution::min_window("a".into(), "a".into()),
            "a".to_string()
        );
        assert_eq!(
            Solution::min_window("a".into(), "aa".into()),
            "".to_string()
        );
    }
}
