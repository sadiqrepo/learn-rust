fn main(){

    // Assignment operators
    let c = 2;
   let d = c;
   println!("d = c");
   println!("Value of c:{}", c);
   println!("Value of d:{}", d);


   // Compound assignment operator
   let mut a = 2;
   println!("a:{}", a);
   a += 1;
   println!("a+=1:{}", a);
   println!("a:{}", a);
   a -= 1;
   println!("a-=1:{}", a);
   println!("a:{}", a);
   a /= 1;
   println!("a/=1:{}", a);
   println!("a:{}", a);
   a *= 3;
   println!("a*=3:{}", a);

   let mut e = 2;
  let mut f = 3;
  e += e;
  f -= f;
  e *= 1;
  f *= 3;
  e -= 1;
  println!("e: {}", e);
  println!("f: {}", f); 
}