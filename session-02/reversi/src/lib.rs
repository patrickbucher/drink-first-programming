const EMPTY: char = '.';
const BLACK: char = 'X';
const WHITE: char = 'O';
const SIDES: usize = 8;

#[derive(Debug)]
pub struct Board {
    fields: Vec<Vec<char>>,
}

impl Board {
    pub fn new() -> Self {
        let mut rows: Vec<Vec<char>> = Vec::new();
        for i in 0..SIDES {
            let mut cols: Vec<char> = Vec::new();
            for j in 0..SIDES {
                match (i, j) {
                    (3, 3) | (4, 4) => cols.push(WHITE),
                    (3, 4) | (4, 3) => cols.push(BLACK),
                    _ => cols.push(EMPTY),
                }
            }
            rows.push(cols);
        }
        Board { fields: rows }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_board() {
        let board = Board::new();
        assert_eq!(board.fields.len(), SIDES);
        for i in 0..SIDES {
            let row = &board.fields[i];
            assert_eq!(row.len(), SIDES);
            for j in 0..SIDES {
                let col = row[j];
                match (i, j) {
                    (3, 3) | (4, 4) => assert_eq!(col, WHITE),
                    (3, 4) | (4, 3) => assert_eq!(col, BLACK),
                    _ => assert_eq!(col, EMPTY),
                }
            }
        }
    }
}
