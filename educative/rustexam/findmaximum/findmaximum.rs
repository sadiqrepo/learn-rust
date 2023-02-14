
fn find_maximum(arr: &[i32])-> i32 {

    let mut large:i32 = arr[0];
    let mut i = 0;

    while i < arr.len(){
        if large < arr[i]{
            large = arr[i]
        }
        i += 1;        
    }
    large
}

fn main(){
    let arr:[i32;5] = [50,20,10,70,60];
    println!("Largest element in the array is {}", find_maximum(&arr) );
    
}

