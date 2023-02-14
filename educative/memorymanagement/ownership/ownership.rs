
fn pass_string_object(my_string: String) { // my_string comes into scope
    println!("{}", my_string);
} // Here, my_string goes out of scope and `drop` frees the memory

fn pass_integer(my_integer: i32) { // my_integer comes into scope
    println!("{}", my_integer);
} // Here, my_integer goes out of scope

fn move_return_value_str_1()->String {
    let my_string = String::from("Rust");
    my_string
}

fn move_str_2_return_str_2(my_string:String)->String {
    my_string
}


fn main() {

    // Primitives 

    let a = 1; // variable a is the owner of the value 1
    let b = 1; // variable b is the owner of the value 1
    let c = 3; // variable c is the owner of the value 3
    
    println!("a : {}", a);
    println!("b : {}", b);
    println!("c : {}", c);

        
    let d = a; // copy of 'a' is created
    println!("a:{} , d:{}", a, d); // print 'a' and 'd'

    let e = [1,2,3];
    let f = e; // copy of 'a' is created 
    println!("e:{:?} , f:{:?}", e, f); // print 'a' and 'b'

    //Non-primitives
    let g = String::from("Rust");
    let h = g; // moves value of 'a' to 'b'
    //eprintln!("g:{}", g);// Error use of moved value 'g'
    println!("h:{}", h); 

    let i = vec![2, 4, 8];
    let j = i; // move value of 'i' to 'j'
    println!("j : {:?} ", j); // prints 'j'

    let mut k = String::from("Rust"); // define a String and save in 'k'
    let l = k.clone(); // b clones a
    k.push('y');
    println!("k:{} , l:{}", k, l);  // print 'k' and 'l'

    let str = String::from("Rust"); // str comes into scope
                                    // str is a move type

    pass_string_object(str);        // str's value moves into the function...
                                    // ... and becomes in accessible here
    //println!("{}" , str);         // This line will give an error

    let my_int = 10;                // my_int comes into scope

    pass_integer(my_int);          // my_int value is a copy into the function,
                                    // but i32 is a copy type, so can my used
                                    // use my_int if desired
    println!("{}" , my_int);        //my_int is still accessible as the value was copied



    let str1 = move_return_value_str_1();
    println!("The function gives ownership to string by returning a value. \nString 1: {}",str1);

    let str2 = String::from("Rust Programming");
    println!("This is a string declared. \nString 2: {}",str2);

    let str3 = move_str_2_return_str_2(str2);
    println!("string 2 passes to the function and returns its value to string 3. \nString 3: {}",str3)
 
  }// value a, b, c are out of scope outside this block