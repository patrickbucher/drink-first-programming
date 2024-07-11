const EMPTY: char = '.';
const SIDES: usize = 8;

#[derive(Debug)]
pub struct Board {
    fields: Vec<Vec<char>>,
}

impl Board {
    pub fn new() -> Self {
        let mut rows: Vec<Vec<char>> = Vec::new();
        for _ in 0..SIDES {
            let mut cols: Vec<char> = Vec::new();
            for _ in 0..SIDES {
                cols.push(EMPTY);
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
        for row in board.fields {
            assert_eq!(row.len(), SIDES);
        }
    }
}
