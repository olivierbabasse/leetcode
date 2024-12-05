//! <https://leetcode.com/problems/swap-adjacent-in-lr-string/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn can_transform(start: String, result: String) -> bool {
        let (start, result) = (start.as_bytes(), result.as_bytes());
        let (slen, mut sindex, mut tindex) = (start.len(), 0, 0);

        while sindex < slen || tindex < slen {
            while sindex < slen && start[sindex] == b'X' {
                sindex += 1;
            }
            while tindex < slen && result[tindex] == b'X' {
                tindex += 1;
            }
            if sindex == slen || tindex == slen {
                return sindex == slen && tindex == slen;
            }
            if start[sindex] != result[tindex]
                || (start[sindex] == b'L' && sindex < tindex)
                || (start[sindex] == b'R' && sindex > tindex)
            {
                return false;
            }
            sindex += 1;
            tindex += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::can_transform(
            "RXXLRXRXL".into(),
            "XRLXXRRLX".into()
        ));
    }
}
