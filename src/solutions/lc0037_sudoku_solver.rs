//! <https://leetcode.com/problems/sudoku-solver/>

const N: usize = 9;

struct SudokuSolver {
    rows: Vec<[bool; N]>,
    cols: Vec<[bool; N]>,
    boxes: Vec<[bool; N]>,
}

impl SudokuSolver {
    fn new() -> Self {
        Self {
            rows: vec![[false; N]; N],
            cols: vec![[false; N]; N],
            boxes: vec![[false; N]; N],
        }
    }

    fn is_valid_place(&self, row: usize, col: usize, num: usize) -> bool {
        let box_index = (row / 3) * 3 + (col / 3);
        !self.rows[row][num] && !self.cols[col][num] && !self.boxes[box_index][num]
    }

    fn place_number(&mut self, board: &mut [Vec<char>], row: usize, col: usize, num: usize) {
        board[row][col] = (num as u8 + b'1') as char;
        self.rows[row][num] = true;
        self.cols[col][num] = true;
        let box_index = (row / 3) * 3 + (col / 3);
        self.boxes[box_index][num] = true;
    }

    fn remove_number(&mut self, board: &mut [Vec<char>], row: usize, col: usize, num: usize) {
        board[row][col] = '.';
        self.rows[row][num] = false;
        self.cols[col][num] = false;
        let box_index = (row / 3) * 3 + (col / 3);
        self.boxes[box_index][num] = false;
    }

    fn solve_sudoku(&mut self, board: &mut [Vec<char>]) -> bool {
        for row in 0..N {
            for col in 0..N {
                if board[row][col] == '.' {
                    for num in 0..N {
                        if self.is_valid_place(row, col, num) {
                            self.place_number(board, row, col, num);
                            if self.solve_sudoku(board) {
                                return true;
                            }
                            self.remove_number(board, row, col, num);
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    fn init_board(&mut self, board: &[Vec<char>]) {
        for row in 0..N {
            for col in 0..N {
                if board[row][col] != '.' {
                    let num = (board[row][col] as u8 - b'1') as usize;
                    self.place_number(&mut board.to_owned(), row, col, num);
                }
            }
        }
    }
}

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn solve_sudoku(board: &mut [Vec<char>]) {
        let mut solver = SudokuSolver::new();
        solver.init_board(board);
        solver.solve_sudoku(board);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                vec!['3', '4', '5', '2', '8', '6', '1', '7', '9']
            ]
        );

        let mut board = vec![
            vec!['.', '.', '9', '7', '4', '8', '.', '.', '.'],
            vec!['7', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '2', '.', '1', '.', '9', '.', '.', '.'],
            vec!['.', '.', '7', '.', '.', '.', '2', '4', '.'],
            vec!['.', '6', '4', '.', '1', '.', '5', '9', '.'],
            vec!['.', '9', '8', '.', '.', '.', '3', '.', '.'],
            vec!['.', '.', '.', '8', '.', '3', '.', '2', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '6'],
            vec!['.', '.', '.', '2', '7', '5', '9', '.', '.'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['5', '1', '9', '7', '4', '8', '6', '3', '2'],
                vec!['7', '8', '3', '6', '5', '2', '4', '1', '9'],
                vec!['4', '2', '6', '1', '3', '9', '8', '7', '5'],
                vec!['3', '5', '7', '9', '8', '6', '2', '4', '1'],
                vec!['2', '6', '4', '3', '1', '7', '5', '9', '8'],
                vec!['1', '9', '8', '5', '2', '4', '3', '6', '7'],
                vec!['9', '7', '5', '8', '6', '3', '1', '2', '4'],
                vec!['8', '3', '2', '4', '9', '1', '7', '5', '6'],
                vec!['6', '4', '1', '2', '7', '5', '9', '8', '3']
            ]
        );
    }
}
