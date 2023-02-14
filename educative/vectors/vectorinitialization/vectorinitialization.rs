

fn main(){
    let vector1 = vec![1,2,3,4,5];
    println!("{:?}",vector1);

    //Accessing an element from index
    println!("vector1[1]: {}",vector1[1]);

    //Accessing an element from index
    //println!("vector1[9]: {}",vector1[9]); 

    match vector1.get(9) {
        Some(x) => println!("Values at given index: {}",x),
        None    => println!("Sorry, you are accessing a value out of bound")
    }

    //print the vector
    println!("\nPrint using debug trait");
    println!("{:?}",vector1);

    println!("\nPrint using for loop");
    let mut index = 0;
    for i in vector1 {
        println!("Value at index {}: {}",index, i);
        index += 1;
    }

    //Initialize a vector object
    let mut vector2 = Vec::new();
    println!("\nEmpty Vector : {:?}", vector2);

    // Push elements to vector
    vector2.push(1);
    vector2.push(2);
    vector2.push(3);
    println!("\nPushed elements : {:?}", vector2);

    // Pop last element from vector
    vector2.pop();
    println!("Vector elements after Pop: {:?}", vector2);

    // Removing an element at given index from vector
    vector2.remove(0);
    println!("Vector elements after remove: {:?}", vector2);

    //Size of vector after push, pop & remove
    println!("Vector size after push, pop & remove: {:?}", vector2);

    // Check if an element is present in vector
    println!("Does Vector elements have 1: {}", vector2.contains(&1));

}

