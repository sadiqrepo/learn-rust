
fn main() {

    let mut s1 = String::from("Rust");
    println!("Length before borrowing: is {}", s1.len());
    // Borrowing a mutable reference
    let len = calculate_length(&mut s1);
    println!("Length after borrowing is {}",len);
}

fn calculate_length(x: &mut String) -> usize {
    x.push_str(" is cool!");
    x.len()
}
