//! <https://leetcode.com/problems/distinct-subsequences/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    fn rec(
        s: &[u8],
        sindex: usize,
        t: &[u8],
        tindex: usize,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if tindex >= t.len() {
            return 1;
        }
        if sindex >= s.len() {
            return 0;
        }
        if let Some(val) = cache.get(&(sindex, tindex)) {
            return *val;
        }
        let mut res = Self::rec(s, sindex + 1, t, tindex, cache);
        if s[sindex] == t[tindex] {
            res += Self::rec(s, sindex + 1, t, tindex + 1, cache);
        }
        cache.insert((sindex, tindex), res);
        res
    }

    pub fn num_distinct(s: String, t: String) -> i32 {
        Self::rec(s.as_bytes(), 0, t.as_bytes(), 0, &mut HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_distinct("rabbbit".into(), "rabbit".into()), 3);
        assert_eq!(Solution::num_distinct("babgbag".into(), "bag".into()), 5);
    }
}
