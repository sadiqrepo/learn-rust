

fn main(){

    let arr:[i32;4] = [1, 2, 3, 4];
    let borrow_array = &arr[0..2];

    println!("Array: {:?}",arr);
    println!("Slice Array: {:?}",borrow_array);

    let str = String::from("Rust Programming");
    let borrow_string = &str[0..2];

    println!("String: {:?}",str);
    println!("Slice String: {:?}",borrow_string);

    let my_vec = vec![1, 2, 3, 4];
    let borrow_vec = &my_vec[0..2];

    println!("Vector: {:?}",my_vec);
    println!("Slice Vector: {:?}",borrow_vec);
}