fn main(){

    // Create tuple syntax1
    let tuple1 = ("Virat", 'c',18);
    let tuple2:(&str,char,i32) = ("Dhoni", 'c',7);
    println!("Tuple1:{:?}:",tuple1);
    println!("Tuple2:{:?}:",tuple2);

    //Accessing the tuple values
    let tuple3:(&str,i32,&str,&str) = ("Rohit",30,"5.8ft","78Kgs");
    println!("\nPlayer Name: {}",tuple3.0);
    println!("Player age: {}",tuple3.1);
    println!("Player height: {}",tuple3.2);
    println!("Player weight: {}",tuple3.3);

    //get individual values of tuple
    let (name, age, height, weight) = tuple3;
    println!("\nName: {}", name);
    println!("Age: {}", age);
    println!("Height: {}", height);
    println!("Weight: {}", weight);

    //Modify tuple values
    let mut tuple4:(&str,i32,&str,&str) = ("Hardik",30,"5.8ft","78Kgs");
    println!("\nValues before modifying tuple at index0 & index1 are: {} {}",tuple4.0, tuple4.1);
    tuple4.0 = "Rishab";
    tuple4.1 = 28;
    println!("Values after modifying tuple at index0 & index1 are: {} {}",tuple4.0, tuple4.1);

    // Printing all Tuple elements using debug trait
    println!("\nPlayer data :{:?} ",tuple4);


}