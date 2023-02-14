fn main(){

    //Continue in For loop
    for v in 0..5{
        if v == 2 {
            println!("Continue statement");
            continue;
        }
        println!("var:{}",v)
    }

    //Continue in whle loop
    let mut var = 1; 
    // define a boolean variable
    let mut found = false;
    // define a while loop
    while !found {
      var = var + 1;
      println!("{}", var);
      
      if var == 4 {
          println!("I encoutered a continue statement");
          continue;
        }
        println!("I did not encounter continue statement");
        
        if var == 10{
          found = true;
        }
    }
}