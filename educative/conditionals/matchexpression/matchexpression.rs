fn main(){

    let val = 5;
    let course = "Rust";

    match val {
        1 => println!("Java"),
        2 => println!("Python"),
        3 => println!("C"),
        4 => println!("C++"),
        5 => println!("Rust"),
        6 => println!("Go"),
        _ => println!("Some other value"),
    };

    let found_course = match course {
        "Rust" => "Rust",
        "Java" => "Java",
        "Python" => "Python",
        "Go" => "Go",
        _ => "Unknown language",
    };
    println!("Found course: {}",found_course);
}