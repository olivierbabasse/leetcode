//! <https://leetcode.com/problems/string-compression-iii/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn compressed_string(word: String) -> String {
        word.as_bytes()
            .chunk_by(|a, b| a == b)
            .flat_map(|sequence| {
                sequence
                    .chunks(9)
                    .flat_map(|c| [(b'0' + c.len() as u8) as char, c[0] as char])
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::compressed_string("abcde".into()),
            "1a1b1c1d1e".to_string()
        );
        assert_eq!(
            Solution::compressed_string("aaaaaaaaaaaaaabb".into()),
            "9a5a2b".to_string()
        );
    }
}
