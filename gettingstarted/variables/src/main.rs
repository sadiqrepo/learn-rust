fn main() {

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let x = 5;
    println!("Value of x is : {x}");
   // x = 6;
   // println!("Value of x is : {x}");
   let x = x + 1;
   {
    let x = x * 2;
   println!("The value of x in the inner scope is: {x}");
   }

   println!("Value of x is : {x}");

   let mut a = 10;
   println!("Value of a is : {a}");

   a = 20;
   println!("Value of a is : {a}");
   println!("Three hours in seconds : {THREE_HOURS_IN_SECONDS}");


}
