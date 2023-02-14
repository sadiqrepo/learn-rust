fn main(){
    // define a for loop
  for i in 0..10 {
    print!("i:{} ", i);
    if i == 5 {
      break;
    }
  }

  println!();

  let mut i = 1;
  let found = false;
  // define a while loop
  while !found {
    print!("i:{} ", i);
    if i == 5 {
      break;
    }
    i = i + 1;    
  }
  println!();

  let mut x = 1;
  // define a loop
  loop{
    print!("x:{} ", x);
    if x == 5 {
      break;
    }
    x = x + 1;    
  }

  println!();

}