use connect_four::Board;

fn main() {
    let board = Board::new();
    println!("{}", board.render());
}
