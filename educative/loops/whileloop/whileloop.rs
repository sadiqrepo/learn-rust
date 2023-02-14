fn main(){

    let mut var = 1;
    let mut found = false;

    while !found {
        var = var + 1;
        println!("{}",var);
        if var % 2 == 1 {
            found = true;
        }
        println!("Loop runs");
    }

    //Infinite loop - use loop keyword

    loop {
        var = var + 1;
        print!("{}",var);
        if var == 10 {
            break;
        }
    }
    println!()
}