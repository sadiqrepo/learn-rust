
fn modify_my_array1(mut arr:[i32;5]){
    arr[2] = 8;
    arr[4] = 10;
    println!("Array values inside modify_my_array fn: {:?}", arr);
}

fn modify_my_array2(mut arr:[i32;5])->[i32;5]{
    arr[2] = 8;
    arr[4] = 10;
    arr
}

fn modify_my_array3( arr:&mut [i32;5]){
    arr[2] = 8;
    arr[4] = 10;
    println!("Array values inside modify_my_array fn: {:?}", arr);
}



fn calculate_mean(arr:[i32;5]){
    let mut sum = 0;
    for i in 0..5 {
        sum += arr[i]
    }
    println!("Mean value of array inside calculate_mean fn.: {}", sum/5);

}

fn main(){

    // Pass by value
    let myarray1 = [1,2,3,4,5];
    println!("Pass by value Array in driver function");
    modify_my_array1(myarray1);
    println!("Array in driver function: {:?}", myarray1);
    println!("Returning modified Pass by value Array in driver function: {:?}",modify_my_array2(myarray1));
    calculate_mean(myarray1);
    


    // Pass by reference
    let mut myarray2 = [1,2,3,4,5];
    println!("\nPass by Reference Array in driver function");
    modify_my_array3(&mut myarray2);
    println!("Array in driver function: {:?}", myarray2);

}