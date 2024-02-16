//! <https://leetcode.com/problems/maximum-score-words-formed-by-letters/>

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    fn rec(words: &[String], letter_counts: [usize; 26], score: &[i32]) -> i32 {
        if words.is_empty() {
            return 0;
        }

        let score1 = Self::rec(&words[1..], letter_counts, score);

        let mut letter_counts_clone = letter_counts;
        let mut wscore = 0;
        for &l in words[0].as_bytes() {
            let l = (l - b'a') as usize;
            if letter_counts_clone[l] == 0 {
                return score1;
            }
            letter_counts_clone[l] -= 1;
            wscore += score[l];
        }

        let score2 = wscore + Self::rec(&words[1..], letter_counts_clone, score);

        score1.max(score2)
    }

    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut letter_counts = [0; 26];
        for l in letters {
            letter_counts[(l as u8 - b'a') as usize] += 1;
        }
        Self::rec(&words, letter_counts, &score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_score_words(
                vec!["dog".into(), "cat".into(), "dad".into(), "good".into()],
                vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
                vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ),
            23
        );
    }
}
