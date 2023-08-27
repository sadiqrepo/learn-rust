
fn main() {
    
    // Borrowing an immutable reference
   
    let s1 = String::from("Rust");
    let len = calculate_length(&s1);
    println!("Length of {} is {}",s1, len);
}

fn calculate_length(x: &String) -> usize {
    //x.push_str(" is cool!");
    x.len()
}






