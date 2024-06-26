use akshually::io::prompt_line;
use connect_four::{Board, Outcome, PLAYER_ONE, PLAYER_TWO};

fn main() {
    let mut board = Board::new();
    let mut player = PLAYER_TWO;
    println!("{}", board.render());
    loop {
        if player == PLAYER_ONE {
            player = PLAYER_TWO;
        } else {
            player = PLAYER_ONE;
        }
        let slot: usize = prompt_line(&format!("Player {player}, enter a slot: ")).unwrap();
        match board.drop_stone(player, slot) {
            Ok(outcome) => match outcome {
                Outcome::Win(_) => {
                    println!("{}", board.render());
                    println!("Player {player}, a winner is you!");
                    break;
                }
                _ => {
                    println!("{}", board.render());
                }
            },
            Err(err) => {
                println!("{err}");
                continue;
            }
        }
    }
}
