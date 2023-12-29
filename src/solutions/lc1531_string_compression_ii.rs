//! <https://leetcode.com/problems/string-compression-ii/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    fn rec(
        s: &[u8],
        k: usize,
        index: usize,
        last_char: u8,
        cur_count: usize,
        del_count: usize,
        cache: &mut HashMap<(usize, u8, usize, usize), i32>,
    ) -> i32 {
        if index == s.len() {
            0
        } else if let Some(&res) = cache.get(&(index, last_char, cur_count, del_count)) {
            res
        } else {
            let cur_char = s[index];
            let mut res = 1 + Self::rec(s, k, index + 1, cur_char, 1, del_count, cache);
            if last_char == cur_char {
                let mut comp =
                    Self::rec(s, k, index + 1, cur_char, cur_count + 1, del_count, cache);
                if cur_count == 1 || cur_count == 9 || cur_count == 99 {
                    comp += 1;
                }
                res = res.min(comp);
            }
            if del_count < k {
                res = res.min(Self::rec(
                    s,
                    k,
                    index + 1,
                    last_char,
                    cur_count,
                    del_count + 1,
                    cache,
                ));
            }
            cache.insert((index, last_char, cur_count, del_count), res);
            res
        }
    }

    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        Self::rec(s.as_bytes(), k as usize, 0, 0, 0, 0, &mut HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aaabcccd".into(), 2),
            4
        );
        assert_eq!(
            Solution::get_length_of_optimal_compression("aabbaa".into(), 2),
            2
        );
        assert_eq!(
            Solution::get_length_of_optimal_compression("aaaaaaaaaaa".into(), 0),
            3
        );
    }
}
