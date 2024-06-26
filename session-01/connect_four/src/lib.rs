const WIDTH: usize = 7;
const HEIGHT: usize = 6;

const EMPTY: char = '_';
const PLAYER_ONE: char = 'x';
const PLAYER_TWO: char = 'o';

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
}
