fn main() {
    // Creating a String with ownership
    let mut owner_string = String::from("Hello, ownership!");

    // Using the string and changing it
    println!("Original string: {}", owner_string);

    // Passing ownership to a function
    take_ownership(owner_string);

    // This line will not compile because `owner_string` no longer owns the data
    // println!("String after function call: {}", owner_string);

    // Creating an integer with ownership
    let owner_integer = 42;

    // Borrowing the integer immutably
    borrow_immutable(&owner_integer);

    // Borrowing the integer mutably
    let mut borrowed_integer = owner_integer;
    borrow_mutable(&mut borrowed_integer);

    // The original owner can still use the integer
    println!("Original integer: {}", owner_integer);
}

fn take_ownership(s: String) {
    println!("Received ownership: {}", s);
} // The String `s` goes out of scope and is dropped here

fn borrow_immutable(i: &i32) {
    println!("Borrowed immutably: {}", i);
} // The borrowed reference `i` goes out of scope

fn borrow_mutable(i: &mut i32) {
    *i *= 2;
    println!("Borrowed mutably and modified: {}", i);
} // The borrowed mutable reference `i` goes out of scope

