fn square(n:&mut i32){
    *n = *n * *n;
    println!("The value of n inside function call: {}",n);
}

fn main(){
    let mut n = 4;
    println!("The value of n before function call: {}",n);
    println!("Invoke function");
    square(&mut n);
    println!("The value of n after function call: {}",n);
    println!("Driver function ends")

}