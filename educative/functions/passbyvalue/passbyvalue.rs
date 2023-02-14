
fn square(mut n:i32){
    n = n * n;
    println!("Square value of n is {}: ",n);
}

fn swap(x:i32, y:i32){
    let temp = y;
    let y = x;
    let x = temp;
    println!("Values inside swap x: {}, y: {}",x , y);
}

fn main(){
    let n = 4;
    println!("Value of n before function call: {}",n);
    println!("Invoke square function");
    square(n);
    println!("\nValue of n after function call: {}", n);

    let x = 10;
    let y = 5;
    swap(x,y);
    println!("Values inside driver fn. x: {}, y: {}",x , y);


}