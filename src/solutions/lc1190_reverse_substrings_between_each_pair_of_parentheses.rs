//! <https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut pairs = vec![0; s.len()];
        let mut stack = Vec::new();
        for (index, &b) in s.as_bytes().iter().enumerate() {
            if b == b'(' {
                stack.push(index);
            } else if b == b')' {
                let i = stack.pop().unwrap();
                pairs[i] = index;
                pairs[index] = i;
            }
        }

        let mut res = String::new();
        let (mut index, mut direction) = (0i32, 1);
        while index < s.len() as i32 {
            let b = s.as_bytes()[index as usize];
            if b == b'(' || b == b')' {
                index = pairs[index as usize] as i32;
                direction *= -1;
            } else {
                res.push(b as char);
            }
            index += direction;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::reverse_parentheses("(abcd)".into()),
            "dcba".to_string()
        );
        assert_eq!(
            Solution::reverse_parentheses("(u(love)i)".into()),
            "iloveu".to_string()
        );
        assert_eq!(
            Solution::reverse_parentheses("(ed(et(oc))el)".into()),
            "leetcode".to_string()
        );
    }
}
