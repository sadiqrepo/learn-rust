fn main(){

    test(2);
    test(-1);
    test(5);

}

fn test(n:i32) {
    // Write code here! 
 
  let mut factorial = 1;
 
    if n < 0 {
       println!("0");
    }
    else if n == 0 {
       println!("1");
    }
    else
    {
       for i in 1..n + 1 {
          factorial = factorial * i
       }
       println!("{}", factorial);
    }
 
       
   
   
 }