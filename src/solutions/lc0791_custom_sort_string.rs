//! <https://leetcode.com/problems/custom-sort-string/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut positions = [-1; 26];
        order
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(index, &b)| positions[(b - b'a') as usize] = index as i32);
        let mut res = s.as_bytes().to_owned();
        res.sort_unstable_by(|&a, &b| {
            let pos_a = positions[(a - b'a') as usize];
            let pos_b = positions[(b - b'a') as usize];
            pos_a.cmp(&pos_b)
        });
        std::str::from_utf8(&res[..]).unwrap_or_default().into()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::custom_sort_string("cba".into(), "abcd".into()),
            "dcba".to_string()
        );
        assert_eq!(
            Solution::custom_sort_string("bcafg".into(), "abcd".into()),
            "dbca".to_string()
        );
    }
}
