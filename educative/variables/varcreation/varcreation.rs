fn main(){
    let language = "Rust";
    //language = "Not Rust";
    // The above statement will not work as variables are immutable by default in Rust.
    println!("Language: {}",language);

    let mut lang = "Rust";
    println!("Language before mutation: {}",lang);

    lang = "Java";
    println!("Language after mutation: {}",lang);


    //assigning & printing mutliple values
    let (course, category) = ("Rust", "Beginner");
    println!("This is a {} course in {} category", course, category);


    
    

}