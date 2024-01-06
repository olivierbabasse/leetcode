//! <https://leetcode.com/problems/longest-palindromic-substring/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n^2)
impl Solution {
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
    fn test_cases() {
        assert_eq!(
            Solution::longest_palindrome("babad".into()),
            "bab".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".into()),
            "bb".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("eabcb".into()),
            "bcb".to_string()
        );
        assert_eq!(Solution::longest_palindrome("civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth".into()), "ranynar".to_string());
    }
}
