//! <https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();
        let mut res = vec![folder[0].clone()];
        for f in folder.iter().skip(1) {
            let last = format!("{}/", res.last().unwrap());
            if !f.starts_with(&last) {
                res.push(f.clone());
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::remove_subfolders(vec![
                "/a".into(),
                "/a/b".into(),
                "/c/d".into(),
                "/c/d/e".into(),
                "/c/f".into()
            ]),
            vec!["/a".to_string(), "/c/d".to_string(), "/c/f".to_string()]
        );
        assert_eq!(
            Solution::remove_subfolders(vec!["/a".into(), "/a/b/c".into(), "/a/b/d".into(),]),
            vec!["/a".to_string()]
        )
    }
}
