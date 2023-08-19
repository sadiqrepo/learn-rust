
fn goodbye(message: &str){    
    println!("{}", message);
}
fn main() {
    let formal = "Formal: Goodbye.";
    let casual = "Casual: See you later!";
    println!();
    goodbye(formal);
    goodbye(casual);
}
