//! <https://leetcode.com/problems/longest-palindromic-substring/>

use std::collections::HashMap;

struct Solution1 {}

/// time-complexity : O(n^3)
/// space-complexity : O(1)
impl Solution1 {
    fn is_palindrome(
        s: &[u8],
        begin: usize,
        end: usize,
        memo: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        if let Some(res) = memo.get(&(begin, end)) {
            return *res;
        }

        let mid = (end - begin) / 2;
        for i in 0..=mid {
            if s[begin + i] != s[end - i] {
                memo.insert((begin, end), false);
                return false;
            }
        }
        memo.insert((begin, end), true);
        true
    }

    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let len = s.len();
        let mut memo = HashMap::new();

        let mut max_len = 0;
        let mut begin = 0;
        for start in 0..len {
            for i in 0..=start {
                for j in start..len {
                    if j - i + 1 > max_len && Self::is_palindrome(s, i, j, &mut memo) {
                        max_len = j - i + 1;
                        begin = i;
                    }
                }
            }
        }

        String::from_utf8(s[begin..begin + max_len].into()).unwrap()
    }
}

struct Solution2 {}

/// time-complexity : O(n^2)
/// space-complexity : O(n^2)
impl Solution2 {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let len = s.len();
        let mut memo = HashMap::new();

        let mut pbegin = 0;
        let mut plen = 1;

        for i in 0..len {
            memo.insert((i, i), true);
        }

        for i in 0..len - 1 {
            if s[i] == s[i + 1] {
                memo.insert((i, i + 1), true);
                if plen < 2 {
                    pbegin = i;
                    plen = 2;
                }
            }
        }

        for diff in 2..len {
            for i in 0..len - diff {
                let j = i + diff;
                if s[i] == s[j] && memo.get(&(i + 1, j - 1)).unwrap_or(&false) == &true {
                    memo.insert((i, j), true);
                    if j - i + 1 > plen {
                        pbegin = i;
                        plen = j - i + 1;
                    }
                }
            }
        }

        String::from_utf8(s[pbegin..pbegin + plen].into()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_1() {
        assert_eq!(
            Solution1::longest_palindrome("babad".into()),
            "bab".to_string()
        );
        assert_eq!(
            Solution1::longest_palindrome("cbbd".into()),
            "bb".to_string()
        );
        assert_eq!(
            Solution1::longest_palindrome("eabcb".into()),
            "bcb".to_string()
        );
    }

    #[test]
    fn test_cases_2() {
        assert_eq!(
            Solution2::longest_palindrome("babad".into()),
            "bab".to_string()
        );
        assert_eq!(
            Solution2::longest_palindrome("cbbd".into()),
            "bb".to_string()
        );
        assert_eq!(
            Solution2::longest_palindrome("eabcb".into()),
            "bcb".to_string()
        );
        assert_eq!(Solution2::longest_palindrome("civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth".into()), "ranynar".to_string());
    }
}
