//! <https://leetcode.com/problems/defanging-an-ip-address/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace('.', "[.]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::defang_i_paddr("1.1.1.1".into()),
            "1[.]1[.]1[.]1".to_string()
        );
    }
}
