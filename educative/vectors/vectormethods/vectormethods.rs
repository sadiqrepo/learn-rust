fn main(){

    let mut vector1 = vec![1, 2, 3, 4, 5];
    println!("Elements of vector1: {:?}",vector1);
    println!("Capacity of vector1: {:?}",vector1.capacity());
    println!("Length of vector1: {:?}", vector1.len());

    //Adding elements to vector1
    vector1.push(6);
    vector1.push(7);

    println!("\nElements of vector1 after addition: {:?}",vector1);
    println!("Capacity of vector1 after addition: {:?}",vector1.capacity());
    println!("Length of vector1 after addition: {:?}", vector1.len());

    //Remove elements from the end
    vector1.pop();
    vector1.pop();

    println!("\nElements of vector1 after Pop: {:?}",vector1);
    println!("Capacity of vector1 after Pop: {:?}",vector1.capacity());
    println!("Length of vector1 after Pop: {:?}", vector1.len());

    //Remove elements at any given index.
    vector1.remove(0);
    vector1.remove(1);

    println!("\nElements of vector1 after remove: {:?}",vector1);
    println!("Capacity of vector1 after remove: {:?}",vector1.capacity());
    println!("Length of vector1 after remove: {:?}", vector1.len());

     //Adding elements to vector1
     vector1.push(6);
     vector1.push(1);
     vector1.push(7);
     println!("\nElements of vector1 after push: {:?}",vector1);
     println!("Capacity of vector1 after push: {:?}",vector1.capacity());
     println!("Length of vector1 after push: {:?}", vector1.len());


    //Use iter function to get the index of an element and remove it
    let value = 1;
    let index = vector1.iter().position(|&r| r == value).unwrap();
    println!("Index of value: {}", index);
    vector1.remove(index);

    println!("\nElements of vector1 after remove: {:?}",vector1);
    println!("Capacity of vector1 after remove: {:?}",vector1.capacity());
    println!("Length of vector1 after remove: {:?}", vector1.len());

    //Iterating through a vector

    let mut inx = 0;
    for i in vector1.iter() {
        println!("Element at index {}: {}",inx, i);
        inx += 1;
    }

    // Loop & Mutate values
    for i in vector1.iter_mut(){
        *i = *i * 3;
    }

    println!("\nElements of vector1 after remove: {:?}",vector1);
    println!("Capacity of vector1 after remove: {:?}",vector1.capacity());
    println!("Length of vector1 after remove: {:?}", vector1.len());

    // Slicing a vector
   let slice:&[i32] = &vector1[2..4];
   // print the vector
   println!("Slice of the vector : {:?}",slice);





}