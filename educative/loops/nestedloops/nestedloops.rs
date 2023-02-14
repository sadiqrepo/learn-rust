fn main(){
    for i in 1..5{ //outer loop
        println!("\nMultiplication Table of : {}", i);
         for j in 1..5 { // inner loop
             println!("{} * {} = {}", i, j, i * j);
         }
         
       }

       'outer:for x in 1..5 { //outer loop
        println!("Muliplication Table : {}", x);
       'inner:for y in 1..5 { // inner loop
            if x == 3 { continue 'outer; } // Continues the loop over `x`.
            if y == 2 { continue 'inner; } // Continues the loop over `y.
            println!("{} * {} = {}", x, y, x * y);
       }
     }

 
}