fn main(){

    // Array creation

    let array1:[i32;4] = [1,2,3,4];
    let array2 = [1 ; 4];

    //Accessing & Printing the array element
    println!("Value at array1[2] is: {}", array1[2]);
    println!("Value at array2[1] is: {}", array2[1]);
    println!("Value at array2[3] is: {}", array2[3]);

    // Mutating an array
    let mut array3:[i32;4] = [1,2,3,4];
    println!("Value at array3[3] is before mutation: {}", array3[3]);

    array3[3] = 5;
    println!("Value at array3[3] is after mutation: {}", array3[3]);

    // Printing all array elements using debug trait

    println!("\nPrint using debug trait");
    println!("Array: {:?}",array3);

    println!("Length of the array3: {}",array3.len());

    // Slice array
    let slice1:&[i32] = &array3;
    let slice2:&[i32] = &array3[1..4];

    //Print slice
    println!("\nPrint Slice using debug trait");
    println!("Slice1: {:?}",slice1);
    println!("Slice2: {:?}",slice2);




}