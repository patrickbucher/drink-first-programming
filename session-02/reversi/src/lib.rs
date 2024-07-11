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

    pub fn is_valid_move(&self, row: usize, col: usize, player: char) -> bool {
        let field = self.fields[row][col];
        if field != EMPTY {
            return false;
        }
        let directions: &[(isize, isize)] = &[
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];
        let opponent = match player {
            BLACK => WHITE,
            WHITE => BLACK,
            _ => return false,
        };
        for (rd, cd) in directions {
            let mut has_visited_opponent = false;
            let mut r = row as isize + rd;
            let mut c = col as isize + cd;
            let sides = SIDES as isize;
            while r >= 0 && r < sides && c >= 0 && c < sides {
                let field = self.fields[r as usize][c as usize];
                if field == player && has_visited_opponent {
                    return true;
                } else if field == opponent {
                    has_visited_opponent = true;
                    r += rd;
                    c += cd;
                } else {
                    break;
                }
            }
        }
        return false;
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

    #[test]
    fn validate_first_move() {
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
        assert_eq!(board.is_valid_move(0, 0, BLACK), false);
        assert_eq!(board.is_valid_move(4, 4, BLACK), false);
        assert_eq!(board.is_valid_move(2, 2, BLACK), false);
        assert_eq!(board.is_valid_move(2, 3, BLACK), true);
        assert_eq!(board.is_valid_move(2, 4, BLACK), false);
        assert_eq!(board.is_valid_move(2, 4, WHITE), true);
        assert_eq!(board.is_valid_move(2, 5, BLACK), false);
        assert_eq!(board.is_valid_move(2, 5, WHITE), false);
        assert_eq!(board.is_valid_move(3, 5, BLACK), false);
        assert_eq!(board.is_valid_move(3, 5, WHITE), true);
        assert_eq!(board.is_valid_move(4, 5, BLACK), true);
        assert_eq!(board.is_valid_move(4, 5, WHITE), false);
        assert_eq!(board.is_valid_move(5, 5, BLACK), false);
        assert_eq!(board.is_valid_move(5, 5, WHITE), false);
        assert_eq!(board.is_valid_move(5, 4, BLACK), true);
        assert_eq!(board.is_valid_move(5, 4, WHITE), false);
        assert_eq!(board.is_valid_move(5, 3, BLACK), false);
        assert_eq!(board.is_valid_move(5, 3, WHITE), true);
        assert_eq!(board.is_valid_move(5, 2, BLACK), false);
        assert_eq!(board.is_valid_move(5, 2, WHITE), false);
        assert_eq!(board.is_valid_move(4, 2, BLACK), false);
        assert_eq!(board.is_valid_move(4, 2, WHITE), true);
        assert_eq!(board.is_valid_move(3, 2, BLACK), true);
        assert_eq!(board.is_valid_move(3, 2, WHITE), false);
    }
}
