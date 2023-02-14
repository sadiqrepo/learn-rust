
enum KnightMove{
    Horizontal, Vertical
}

fn print_direction(direction:KnightMove){

    match direction {
        KnightMove::Horizontal => {
            println!("Move Horizontally two steps one step vertical");
        },
        KnightMove::Vertical => {
            println!("Move Vertically two steps one step horizontal");
        },
    }
}

fn main(){
    let knight1 = KnightMove::Horizontal;
    let knight2 = KnightMove::Vertical;

    print_direction(knight1);
    print_direction(knight2);
}