
fn param_func(param1:i32, param2:i32){
    println!("The First value passed inside this function: {}", param1);
    println!("The Second value passed inside this function: {}", param2);
}

fn main(){
    let value1 = 10;
    let value2 = 20;
    param_func(value1, value2);
    println!("Driver function ended")
}