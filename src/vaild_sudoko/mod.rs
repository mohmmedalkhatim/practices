use std::collections::HashMap;
#[derive(Debug)]
pub struct Element {
    col: usize,
    row: usize,
    cubic: usize,
}
pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut hash: HashMap<char, Element> = HashMap::new();

        for i in 0..9 {
            for j in 0..9 {
                let current = board[i][j];
                let cubic = (i / 3) * 3 + (j / 3);
                if current != '.' {
                    let _ = hash.entry(current).or_insert(Element {
                        col: j,
                        row: i,
                        cubic,
                    });
                }
            }
        }
        let values: Vec<&char> = hash.keys().collect();
        let mut start = 0;
        let mut end = start + 1;
        while start < 9 {
            while end < 9 {
                if end != start {
                    let prv = hash.get(values[start]).unwrap();
                    let next = hash.get(values[end]).unwrap();
                    println!("{}\t{}",values[start],values[end]);
                    if values[start] == values[end] {
                        if prv.col == next.col || next.row == prv.row || next.cubic == prv.cubic {
                            return false;
                        }
                    }
                }
                end += 1;
            }
            end = 0;
            start += 1
        }

        return true;
    }
}
