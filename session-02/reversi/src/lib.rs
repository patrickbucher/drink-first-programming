use std::fmt::{self, Display, Formatter};

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

    pub fn from(fields: &Vec<Vec<u8>>, (e, b, w): (u8, u8, u8)) -> Self {
        let mut chars: Vec<Vec<char>> = Vec::new();
        for i in 0..fields.len() {
            let mut row: Vec<char> = Vec::new();
            for j in 0..fields[i].len() {
                let field = fields[i][j];
                let chr: char = if field == e {
                    EMPTY
                } else if field == b {
                    BLACK
                } else if field == w {
                    WHITE
                } else {
                    EMPTY
                };
                row.push(chr);
            }
            chars.push(row);
        }
        Board { fields: chars }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "  ")?;
        for i in 0..self.fields.len() {
            write!(f, "{i} ")?;
        }
        write!(f, "\n")?;
        for i in 0..self.fields.len() {
            let row = &self.fields[i];
            let r = (('a' as u8) + (i as u8)) as char;
            write!(f, "{r} ")?;
            for j in 0..row.len() {
                let col = row[j];
                write!(f, "{col} ")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
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

    #[test]
    fn init_board_from() {
        let fields = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 2, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 2, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let board = Board::from(&fields, (0, 1, 2));
        assert_eq!(board.fields[3][3], WHITE);
        assert_eq!(board.fields[4][4], WHITE);
        assert_eq!(board.fields[3][4], BLACK);
        assert_eq!(board.fields[4][3], BLACK);
    }
}
