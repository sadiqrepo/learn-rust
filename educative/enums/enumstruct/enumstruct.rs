#[derive(Debug)]

enum KnightSteps{
    Horizontal, Vertical
}

#[derive(Debug)]
#[allow(dead_code)]

struct Player {
    color:String,
    knight:KnightSteps,
}

fn main(){

    let p1 = Player {
        color:String::from("Black"),
        knight:KnightSteps::Horizontal,
    };

    let p2 = Player {
        color:String::from("White"),
        knight:KnightSteps::Vertical,
    };

    println!("{:?}",p1);
    println!("{:?}",p2);
}