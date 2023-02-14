fn arr_square() -> [i32;5] {
    let mut square:[i32;5] = [1, 2, 3, 4, 5]; // mutable array 
    for i in 0..5 {  // compute the square of each element
        square[i] = square[i] * square[i];
    }
    square
}
fn main(){
    println!("Updated Array : {:?}",arr_square());
}