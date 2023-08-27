// A function that returns the longest of two string slices
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = String::from("Hello, ");
    let result;
    
    {
        let string2 = String::from("world");

        // Borrowed values must have a lifetime that fits within the scope
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", result);
    } // string2 goes out of scope here

    // This line will not compile because string2 is no longer valid
    // println!("The longest string is: {}", result);
}


fn main() {
    let data = vec![1, 2, 3];
    let reference = &data; // Reference points to data
    
    // 'data' goes out of scope, making 'reference' a dangling reference
    
    println!("{:?}", *reference); // Using 'reference' here leads to undefined behavior
}

