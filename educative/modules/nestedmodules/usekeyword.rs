
mod nestedmodules;
use nestedmodules::chapter::lesson::heading;

fn main(){
    println!("Calling nested mod function");
    heading::illustration();
}