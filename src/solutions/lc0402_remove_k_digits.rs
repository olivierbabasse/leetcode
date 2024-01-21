//! <https://leetcode.com/problems/remove-k-digits/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut k = k as usize;
        let mut stack = Vec::<char>::new();

        for c in num.chars() {
            while k > 0 && !stack.is_empty() && stack.last().unwrap() > &c {
                stack.pop();
                k -= 1;
            }

            if stack.is_empty() && c == '0' {
                continue;
            }

            stack.push(c);
        }

        if stack.len() <= k {
            return "0".into();
        }

        stack[..stack.len() - k].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::remove_kdigits("1432219".into(), 3),
            "1219".to_string()
        );
        assert_eq!(
            Solution::remove_kdigits("10200".into(), 1),
            "200".to_string()
        );
        assert_eq!(Solution::remove_kdigits("10".into(), 2), "0".to_string());
    }
}
