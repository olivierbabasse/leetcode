//! <https://leetcode.com/problems/subrectangle-queries/>

struct SubrectangleQueries {
    values: Vec<Vec<i32>>,
}

impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self { values: rectangle }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for i in row1..=row2 {
            for j in col1..=col2 {
                self.values[i as usize][j as usize] = new_value
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.values[row as usize][col as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let mut obj = SubrectangleQueries::new(vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3]]);
        obj.update_subrectangle(1, 1, 1, 1, 9);
        assert_eq!(obj.get_value(1, 1), 9);
    }
}
