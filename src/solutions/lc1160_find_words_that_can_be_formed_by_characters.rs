//! <https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/>

use std::collections::HashMap;

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut total = 0;
        let mut frequencies = HashMap::<char, usize>::new();
        chars.chars().for_each(|c| {
            *frequencies.entry(c).or_default() += 1;
        });

        'outer: for w in words {
            let mut word_frequencies = HashMap::<char, usize>::new();
            w.chars().for_each(|c| {
                *word_frequencies.entry(c).or_default() += 1;
            });

            for (c, freq) in word_frequencies {
                if frequencies.get(&c).unwrap_or(&0) < &freq {
                    continue 'outer;
                }
            }

            total += w.len();
        }

        total as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::count_characters(
                vec!["cat".into(), "bt".into(), "hat".into(), "tree".into()],
                "atach".into()
            ),
            6
        );
        assert_eq!(
            Solution::count_characters(
                vec!["hello".into(), "world".into(), "leetcode".into()],
                "welldonehoneyr".into()
            ),
            10
        );
    }
}
