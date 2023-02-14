
//Rule 1: Invoking a public function directly
mod moddemo {

    
    pub fn print_out(){
        println!("This is a function inside a moddemo module!");
    }
    
    pub fn my_public_function(){
        //! also works without writing self i.e.
        //! my_private_function();
        println!("\nHi,I'm a public function within the moddemo module");
        println!("I'll invoke private function within the moddemo module");

        //Rule 2.1: Invoking a private function indirectly through a public function
        self::my_private_function(); 
               
      }
    
    fn my_private_function(){
        println!("\nHi, I'm a private function within the moddemo module");

        //Rule 2.3: Access a Root Function
        super::print_rootfn();
      }


    pub mod innermoddemo {

        pub fn my_pub_fn(){
            println!("\nHi,I'm a public function within the innermoddemo module");
            println!("I'll invoke private function from the moddemo module");

            //Rule 2.2: Access a Private Function through a Child Module
            super::my_private_function();
           
        }
    }
}

fn print_rootfn(){
    println!("\nHi, I'm a root function, I can be invoked inside module using super");
}



fn main(){
    println!("Lets go inside modules!");
    moddemo::print_out();
    moddemo::my_public_function();
    moddemo::innermoddemo::my_pub_fn();
}