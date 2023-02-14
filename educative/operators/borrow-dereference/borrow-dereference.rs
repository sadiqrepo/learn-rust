fn main(){

    let x1 = 10;
    let mut y1 = 20;

    //Immutable reference to a variable
    let x2 = &x1;
    
    println!("Value of x1:{}",x1);
    // Value for x2 remains same as of x1 as it is immutably(Shared) borrowed
    println!("Value of x2:{}",x2);

    //Mutable reference to a variable
    let y2 = &mut y1;
    println!("Value of y2:{}",y2);

    //dereferencing
    *y2 = 5;

    // Value of y2 is updated
    println!("Value of y2:{}", y2);

    // Value of y1 is updated too as it is mutably borrowed
    println!("Value of y1:{}", y1);



}