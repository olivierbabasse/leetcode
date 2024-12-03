//! <https://leetcode.com/problems/adding-spaces-to-a-string/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut res = String::new();
        let mut index = 0;

        for space in spaces {
            while index < space {
                res.push(s.as_bytes()[index as usize] as char);
                index += 1;
            }
            res.push(' ');
        }
        res.push_str(&s[index as usize..]);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::add_spaces("LeetcodeHelpsMeLearn".into(), vec![8, 13, 15]),
            "Leetcode Helps Me Learn".to_string()
        );
        assert_eq!(
            Solution::add_spaces("icodeinpython".into(), vec![1, 5, 7, 9]),
            "i code in py thon".to_string()
        );
        assert_eq!(
            Solution::add_spaces("spacing".into(), vec![0, 1, 2, 3, 4, 5, 6]),
            " s p a c i n g".to_string()
        );
    }
}
