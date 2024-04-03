//! <https://leetcode.com/problems/word-search/>

struct Solution {}

/// time-complexity : O(m*n*4^len(word))
/// space-complexity : O(len(word))
impl Solution {
    fn rec(board: &mut [Vec<char>], word: &[char], pos: usize, i: i32, j: i32) -> bool {
        if pos >= word.len() {
            return true;
        }

        if i < 0
            || i >= board.len() as i32
            || j < 0
            || j >= board[0].len() as i32
            || board[i as usize][j as usize] != word[pos]
        {
            return false;
        }

        let temp = board[i as usize][j as usize];
        board[i as usize][j as usize] = 0 as char;
        if Self::rec(board, word, pos + 1, i + 1, j)
            || Self::rec(board, word, pos + 1, i - 1, j)
            || Self::rec(board, word, pos + 1, i, j + 1)
            || Self::rec(board, word, pos + 1, i, j - 1)
        {
            true
        } else {
            board[i as usize][j as usize] = temp;
            false
        }
    }

    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        for i in 0..board.len() as i32 {
            for j in 0..board[0].len() as i32 {
                if Self::rec(&mut board, &word.chars().collect::<Vec<_>>(), 0, i, j) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".into()
        ));
    }
}
