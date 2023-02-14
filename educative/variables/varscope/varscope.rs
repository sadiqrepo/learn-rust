fn main(){

    let outer_variable = 100;
    {
        let inner_variable = 200;
        // Outer variable can be accessed inside a scope
        println!("Outer variable: {}",inner_variable);
        println!("Inner variable: {}",outer_variable);
    }
        // Inner variable cannot be accessed outside its scope
        //println!("Outer variable: {}",inner_variable);
        println!("Inner variable: {}",outer_variable);
}