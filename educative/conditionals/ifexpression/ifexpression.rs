fn main(){

    let learn_lang1 = "Rust";
    let learn_lang2 = "Java";
    let learn_lang3 = "Go";


    //if 
    if learn_lang1 == "Rust" {
        println!("\nYou are learning Rust programming language");
    }

    //if..else
    if learn_lang2 == "Rust" {
        println!("\nYou are learning Rust programming language");
    } else {
        println!("\nYou are not learning Rust programming language");
    }

    //if..elseif..else
    if learn_lang3 == "Rust" {
        println!("\nYou are learning Rust programming language");
    } else if learn_lang3 == "Go"{
        println!("\nYou are learning Go programming language");
    } else {
        println!("\nYou are learning a language other than Go & Rust languages");
    }


    //Nested if
    if learn_lang1 == "Rust" {
        if learn_lang3 == "Go"{
                println!("\nYou are learning Rust & Go programming languages");
            } 
        } else {
        println!("\nYou are learning a language other than Go & Rust languages");
    }


    //Shorthand if
    let res = if learn_lang1 == "Rust" {"\nYou are learning Rust lang"} else {"\nYou are not learning Rust lang"};
    println!("{}",res);


    let x = "Rust";

    let y: bool = if x == "Rust" { true } else { false };

    // let z: bool = if x == "Rust" { true; } else { false; };

    println!("x:{}", x);
    println!("y:{}", y);


    //if let expression

    //Define a scrutinee expression
    let course1 = ("Rust", "Beginner", "Course");

    //case 1: Pattern matched
    if let ("Rust", "Beginner", "Course") = course1{
        println!("\nAll value are written to match with scrutinee");
    } else {
        println!("\nPattern not matching")
    }

    //If the first value or second value matches, it can guess the third value.
    if let ("Rust", "Beginner", c) = course1{
        println!("\nWrote first two values in pattern to be matched with the scrutinee expression : {}", c);
    } else {
        println!("\nPattern not matching")
    }

    //If the first value matches, it can guess the other two values.
    if let ("Rust", b, c) = course1{
        println!("\nWrote first two values in pattern to be matched with the scrutinee expression : {} {}", b, c);
    } else {
        println!("\nPattern not matching")
    }


    //case 2: Pattern not matched
    let course2 = ("Rust", "Beginner");
    if let ("Java", b) = course2{
        println!("\nCourse is {}", b);
    } else {
        println!("\nPattern not matching")
    }

    //case 3: Pattern is replaced with _

    //no pattern defined
    if let _ = 10{
        println!("\nirrefutable if-let pattern is always executed");
    } 
    
}