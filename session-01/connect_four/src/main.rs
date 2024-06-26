use connect_four::{Board, PLAYER_ONE, PLAYER_TWO};

fn main() {
    let mut board = Board::new();
    println!("{}", board.render());
    for i in 0..4 {
        println!("{:?}", board.drop_stone(PLAYER_ONE, 1));
        println!("{:?}", board.drop_stone(PLAYER_TWO, 1));
    }
    println!("{}", board.render());
    println!("{:?}", board.drop_stone('$', 3));
    println!("{:?}", board.drop_stone(PLAYER_ONE, 7));
}
