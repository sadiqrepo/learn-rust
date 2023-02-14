
fn factorial(n:i32)-> i32{
    if n == 0 {
        1
    } else {
         n * factorial(n-1)
    }
}

fn main(){
    let n = 4;
    let fact = factorial(n);
    println!("Factorial of {} is: {}", n, fact);
}