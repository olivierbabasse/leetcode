//! <https://leetcode.com/problems/move-pieces-to-obtain-a-string/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let (start, target) = (start.as_bytes(), target.as_bytes());
        let (slen, mut sindex, mut tindex) = (start.len(), 0, 0);

        while sindex < slen || tindex < slen {
            while sindex < slen && start[sindex] == b'_' {
                sindex += 1;
            }
            while tindex < slen && target[tindex] == b'_' {
                tindex += 1;
            }
            if sindex == slen || tindex == slen {
                return sindex == slen && tindex == slen;
            }
            if start[sindex] != target[tindex]
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
        assert!(Solution::can_change("_L__R__R_".into(), "L______RR".into()));
        assert!(!Solution::can_change("R_L_".into(), "__LR".into()));
        assert!(!Solution::can_change("_R".into(), "R_".into()));
    }
}
