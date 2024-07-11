use akshually::io::prompt_line;
use reversi::{Board, BLACK, SIDES, WHITE};

fn main() {
    let mut board = Board::new();
    let mut player = WHITE;
    loop {
        player = match player {
            BLACK => WHITE,
            WHITE => BLACK,
            _ => panic!("invalid player"),
        };
        let (black, white) = board.get_standings();
        println!("black: {}, white: {}", black, white);
        println!("{}", board);
        loop {
            let row_col: String = prompt_line(&format!("Player {player}: ")).unwrap();
            let row_col: Vec<char> = row_col.chars().collect();
            let row: usize = (row_col[0] as u8 - 'a' as u8) as usize;
            let col: usize = (row_col[1] as u8 - '0' as u8) as usize;
            if row > SIDES || col > SIDES {
                continue;
            }
            if board.is_valid_move(row, col, player) {
                board.apply_move(row, col, player);
                break;
            }
        }
    }
}
