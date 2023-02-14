fn main(){

    //for loop
    for i in 0..5 {
        print!("{} ", i);
    }

    println!("\n");

    //for loop enumeration
    for (count, variable) in (7..10).enumerate(){        
        println!("Count = {}, Variable = {}", count, variable);
    }
}