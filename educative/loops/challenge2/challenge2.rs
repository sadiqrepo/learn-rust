fn main(){
    test(21)
}
fn test(mut x:i32) {
    // Write code here!
    let mut counter = 0;
    while x >= 0 {
        counter = counter + 1;
        x = x - 3;
    }
    println!("{}", counter)
}