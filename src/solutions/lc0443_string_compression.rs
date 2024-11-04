//! <https://leetcode.com/problems/string-compression/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        *chars = chars
            .chunk_by(|a, b| a == b)
            .flat_map(|sequence| {
                std::iter::once(sequence[0]).chain(if sequence.len() == 1 {
                    vec![]
                } else {
                    sequence.len().to_string().chars().collect()
                })
            })
            .collect::<Vec<_>>();
        chars.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']),
            6
        );
        assert_eq!(Solution::compress(&mut vec!['a']), 1);
    }
}
