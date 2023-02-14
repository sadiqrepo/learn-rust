fn main(){

    // string literal
    let language:&str = "Rust";
    println!("String literal: {}", language);
    println!("Length of string: {}", language.len());

    //String Objects

    // create an empty string
    let course1 = String::new();

    // Convert string literal to string object using .to_string();
    let course2 = course1.to_string();

    // print the string object
    println!("\nThis is my empty string: {}",course2);
    // print the length of the string object
    println!("This is the length of my empty string: {}",course2.len());

    // Create string literal
    let course3 = "Rust programming language";

    //Convert string literal to string object using .to_string()
    let course4 = course3.to_string();

    // print the string object
    println!("\nThis is my string literal: {}",course4);
    // print the length of the string object
    println!("This is the length of my string literal: {}",course4.len());


    // Define a string object using from method
    let course5 = String::from("Rust for Beginners");
    // print the string object
    println!("\nThis is my string object: {}",course5);
    // print the length of the string object
    println!("This is the length of my string object: {}",course5.len());

}