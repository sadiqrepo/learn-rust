#[derive(Debug)]

enum KnightMode {
    Horizontal, Vertical,
}

use KnightMode::*;
fn main(){
    let hz_move = Horizontal;
    let vt_move = Vertical;

    println!("{:?}",hz_move);
    println!("{:?}",vt_move);
}