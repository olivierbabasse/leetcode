//! <https://leetcode.com/problems/sort-characters-by-frequency/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut freqs = vec![(0, 0); 256];
        s.bytes().for_each(|b| {
            freqs[b as usize].0 += 1;
            freqs[b as usize].1 = b
        });
        freqs.retain(|f| f.0 > 0);
        freqs.sort_unstable();
        freqs
            .iter()
            .rev()
            .map(|f| std::iter::repeat(f.1 as char).take(f.0).collect::<String>())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::frequency_sort("tree".into()), "eetr".to_string());
    }
}
