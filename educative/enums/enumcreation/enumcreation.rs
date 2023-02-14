#[derive(Debug)]

enum KnightMoves {
    Horizontal, Vertical
}

fn main(){

    let horizontal_move = KnightMoves::Horizontal;
    let vertical_move = KnightMoves::Vertical;

    println!("Move 1: {:?}",horizontal_move);
    println!("Move 1: {:?}",vertical_move);
}