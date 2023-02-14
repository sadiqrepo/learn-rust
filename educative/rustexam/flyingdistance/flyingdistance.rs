struct Creatures{
    time : i32,
}
impl Creatures {
   fn fly(&self) {
    format!("{}",self.time)
   }
}
trait Bees {
   fn fly(&self); 
}
trait Birds {
   fn fly(&self); 
}
impl Bees for Creatures {
   fn fly(&self) {
       println!("None");
       // your code goes here!
       println!(self.fly());
   }
}
impl Birds for Creatures {
   fn fly(&self) {
       println!("None");
       // your code goes here!
       println!(self.fly());
   }
}

fn main() {
    //initialize
    let c1 = Creatures  {
       time:2,
    };

    let c2 = Creatures  {
        time:10,
     };
   
    
 }