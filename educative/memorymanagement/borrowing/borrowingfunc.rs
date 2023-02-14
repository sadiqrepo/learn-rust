
fn example(a:i32, b:& i32, c:&mut i32){
    println!("a:{}, b:{}, c:{}",a, b, c);
    *c = 9;
}

fn main(){
    let a = 1;
    let b = 2;
    let mut c = 3;
    example(a, &b, &mut c);
    println!("a:{}, b:{}, c:{}",a, b, c);
}