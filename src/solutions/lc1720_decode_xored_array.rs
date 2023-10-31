//! <https://leetcode.com/problems/decode-xored-array/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut decoded = vec![first];
        let mut tmp = first;
        for e in encoded.into_iter() {
            tmp ^= e;
            decoded.push(tmp);
        }
        decoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
        assert_eq!(Solution::decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4]);
    }
}
