fn square(n:i32)->i32{
    println!("The value of n inside function : {}", n);
    let m = n * n;
    m // return the square of the number n
  }  

fn return_square(n:i32)->i32{
    println!("Value of n inside function call: {}",n);
    return n * n;
} 

fn main() {
    let n = 4;
    println!("Value of n before function call: {}", n);
    println!("Invoke function");
    println!("Value of n after square function call: {}",square(n));
    println!("Value of n after return_square function call: {}", return_square(n));
}

