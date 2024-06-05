//! <https://leetcode.com/problems/find-common-characters/>

struct Solution {}

/// time-complexity : O(n*m)
/// space-complexity : O(1)
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut common = vec![None; 26];
        for w in words {
            let mut freqs = [0; 26];
            for b in w.as_bytes().iter() {
                freqs[(b - b'a') as usize] += 1;
            }
            for i in 0..26 {
                common[i] = Some(common[i].unwrap_or(freqs[i]).min(freqs[i]));
            }
        }
        let mut output = Vec::new();
        for (i, c) in common.into_iter().enumerate() {
            for _ in 0..c.unwrap_or_default() {
                output.push(
                    std::str::from_utf8(&[b'a' + i as u8])
                        .unwrap_or_default()
                        .to_string(),
                );
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::common_chars(vec!["bella".into(), "label".into(), "roller".into()]),
            vec!["e".to_string(), "l".to_string(), "l".to_string()]
        );
    }
}
