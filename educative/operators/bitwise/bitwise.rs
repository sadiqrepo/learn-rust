fn main(){
    let a = 5;
    let b = 6;
    println!("Operand 1: {}, Operand 2: {}", a , b);
    println!("AND: {}", a & b);
    println!("OR: {}", a | b);
    println!("XOR: {}", a ^ b);
    println!("NOT a: {}", !a);
    println!("Left shift: {}", a << 2);
    println!("Right shift: {}", a >> 1);

    let mut c = 1;
  let mut d = 2;
  c = c & d;
  c = c << 1;
  d = d >> 3;
  println!("c: {}", c);
  println!("d: {}", d); 
}