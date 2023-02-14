fn main(){

    let a = 10;
    let b = 5;
    let mut c = true;
    let mut d = true;
    println!("\n***Comparision operators***");
    println!("Operand 1 : {}, Operand 2 : {}",a, b);
    println!("\na > b: {}",a > b);
    println!("a < b: {}",a < b);
    println!("a >= b: {}",a >= b);
    println!("a <= b: {}",a <= b);
    println!("a == b: {}",a == b);
    println!("a != b: {}",a != b);

    
    c = c > d && c < d;
    d = !d;
    println!("c: {}", c);
    println!("d: {}", d); 


}