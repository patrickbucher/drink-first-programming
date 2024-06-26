const CONNECT_N: usize = 4;
const WIDTH: usize = 7;
const HEIGHT: usize = 6;

const EMPTY: char = '_';
pub const PLAYER_ONE: char = 'x';
pub const PLAYER_TWO: char = 'o';

#[derive(Debug)]
pub enum Outcome {
    Win(char),
    Draw,
    Nothing { row: usize, col: usize },
}

pub struct Board {
    fields: Vec<Vec<char>>,
}

impl Board {
    pub fn new() -> Board {
        let mut fields = Vec::new();
        for r in 0..HEIGHT {
            let mut row = Vec::new();
            for c in 0..WIDTH {
                row.push(EMPTY);
            }
            fields.push(row);
        }
        Board { fields }
    }

    pub fn render(&self) -> String {
        let mut output = String::new();
        for i in 1..=WIDTH {
            output.push_str(&format!("{i} "));
        }
        output.push('\n');
        for row in &self.fields {
            for col in row {
                output.push_str(&format!("{col} "));
            }
            output.push('\n');
        }
        output
    }

    pub fn drop_stone(&mut self, stone: char, slot: usize) -> Result<Outcome, String> {
        if stone != PLAYER_ONE && stone != PLAYER_TWO {
            return Err(format!("{stone} is not an allowed stone"));
        }
        match slot {
            1..=WIDTH => {
                let slot_index = slot - 1;
                for r in (0..HEIGHT).rev() {
                    let mut row = &mut self.fields[r];
                    if row[slot_index] == EMPTY {
                        row[slot_index] = stone;
                        return Ok(Outcome::Nothing {
                            row: r,
                            col: slot_index,
                        });
                    }
                }
                Err(format!("slot {slot} is already full"))
            }
            _ => Err(format!("slot {slot} is out of range [{};{}]", 1, WIDTH)),
        }
    }

    fn is_horizontal_win(&self, r: usize, p: char) -> bool {
        let mut matches = 0;
        let row = &self.fields[r];
        println!("{:?}", row);
        for f in row {
            if f == &p {
                matches += 1;
                if matches == CONNECT_N {
                    return true;
                }
            } else {
                matches = 0;
            }
        }
        matches == CONNECT_N
    }

    fn is_vertical_win(&self, c: usize, p: char) -> bool {
        let mut matches = 0;
        for row in &self.fields {
            if row[c] == p {
                matches += 1;
                if matches == CONNECT_N {
                    return true;
                }
            } else {
                matches = 0;
            }
        }
        matches == CONNECT_N
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initially_all_fields_empty() {
        let board = Board::new();
        for row in board.fields {
            for field in row {
                assert_eq!(field, EMPTY);
            }
        }
    }

    #[test]
    fn test_number_of_fields() {
        let board = Board::new();
        assert_eq!(board.fields.len(), HEIGHT);
        for row in board.fields {
            assert_eq!(row.len(), WIDTH);
        }
    }

    #[test]
    fn test_horizontal_not_win() {
        let board = Board::new();
        let actual = board.is_horizontal_win(0, PLAYER_ONE);
        assert_eq!(actual, false);
    }

    #[test]
    fn test_horizontal_win() {
        let mut board = Board::new();
        board.drop_stone(PLAYER_ONE, 1);
        board.drop_stone(PLAYER_ONE, 2);
        board.drop_stone(PLAYER_ONE, 3);
        board.drop_stone(PLAYER_ONE, 4);
        let actual = board.is_horizontal_win(HEIGHT - 1, PLAYER_ONE);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_vertical_not_win() {
        let board = Board::new();
        let actual = board.is_vertical_win(0, PLAYER_TWO);
        assert_eq!(actual, false);
    }

    #[test]
    fn test_vertical_win() {
        let mut board = Board::new();
        board.drop_stone(PLAYER_TWO, 3);
        board.drop_stone(PLAYER_TWO, 3);
        board.drop_stone(PLAYER_TWO, 3);
        board.drop_stone(PLAYER_TWO, 3);
        let actual = board.is_vertical_win(2, PLAYER_TWO);
        assert_eq!(actual, true);
    }
}
