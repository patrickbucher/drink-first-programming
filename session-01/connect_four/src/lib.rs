const WIDTH: usize = 7;
const HEIGHT: usize = 6;

const EMPTY: char = '_';

struct Board {
    fields: Vec<Vec<char>>,
}

impl Board {
    fn new() -> Board {
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
