
fn main() {
    let original_string = String::from("Hello, ownership!");

    // Transferring ownership to another_string
    let another_string = transfer_ownership(original_string);

    // This line will not compile because original_string no longer owns the data
    //println!("Original string: {}", original_string);

    println!("Transferred string: {}", another_string);
}

fn transfer_ownership(s: String) -> String {
    println!("Received ownership: {}", s);
    s // Ownership is transferred out of this function
}
