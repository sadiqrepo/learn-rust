
use std::fmt::Display;

// Generic Function
fn concatenate<T:Display>(t:T, s:T){
    let result = format!("{} {}",t, s);
    println!("{}",result)
}

// Generic Vector

fn print_vec<T:Display>(v: &[T]){
    for i in v.iter(){
        print!("{} ",i);
    }
    println!("");
}

// Generic Struct

struct Rectangle <T> {
    length: T,
    width: T,
}

fn main(){

    // Generic Function
    println!("Passing string params");
    concatenate("Rust", "Programming");
    concatenate(10 as i32, 15 as i32);
    
    //concatenate("Rust", 101 as i32); // different data type params cannot be passed.

    let mut my_vector: Vec<i32> = vec![1,2,3];
    my_vector.push(4);
    println!("{:?}",my_vector);

    // Generic Vector

    println!("\nCall to the function with vector of integers");
    print_vec(&my_vector);

    let my_vector1 = vec!["Rust", "Programming"];
    println!("\nCall to the function with vector of integers");
    print_vec(&my_vector1);

    // Generic struct
    let r1:Rectangle<i32> = Rectangle {
        length:5,
        width:6,
    };

    let r2:Rectangle<f32> = Rectangle {
        length:5.0,
        width:6.0,
    };

    let r3:Rectangle<&str> = Rectangle {
        length: "Rust",
        width: "Programming",
    };

    println!("Width: {}, length:{}", r1.width, r1.length);
    println!("Width: {}, length:{}", r2.width, r2.length);
    println!("Width: {}, length:{}", r3.width, r3.length);


}