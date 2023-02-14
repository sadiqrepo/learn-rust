fn main(){
    test(5, '%', 0);
}

fn test(a: i32, operator: char, b: i32) {
    // Write code here
    match operator {
       '+' => println!("{}", a + b),
       '-' => println!("{}", a - b),
       '*' => println!("{}", a * b),
       '%' => if b == 0{
        println!("Mod 0 is undefined");
    }
    else {
        println!("{}", a % b);
    },
       '/' => if b == 0 {
          println!("Division by 0 is undefined");
       } else {
          println!("{}", a / b)
       },
       _ => println!("invalid operator"),
    };
 }