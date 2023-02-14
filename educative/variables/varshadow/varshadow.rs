fn main(){

    let outer_variable = 250;
    {
        let inner_variable = 150;
        let outer_variable = 350;
        println!("Inner variable: {}",inner_variable);
        println!("Outer variable inside scope: {}",outer_variable);
    }
    println!("Outer variable outside scope: {}",outer_variable);
}