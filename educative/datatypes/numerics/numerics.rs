fn main(){

    // Integer data types
    // Explicitly defining integers
    let a1:i32 = 21;
    let b1:u64 = 25;
    let c1:isize = 27;
    let d1:usize = 29;
    println!("a:{}",a1);
    println!("b:{}",b1);
    println!("c:{}",c1);
    println!("d:{}",d1);

    // Implicitly defining integers
    
    let a2 = 2154;
    let b2 = 2554;
    let c2 = 2754;
    let d2 = 2954;
    println!("a:{}",a2);
    println!("b:{}",b2);
    println!("c:{}",c2);
    println!("d:{}",d2);

    // Floating-point data types
    // Explicitly defining floating points
    let a3:f32 = 23.489;
    let b3:f64 = 25.769;
    println!("a3:{}",a3);
    println!("b3:{}",b3);

    // Implicitly defined floating points

    // below value is deprecated after 15 decimal points
    let a4 = 3.1478114781147811478114781147811478114781;

    // below value is rounded-off after 15 decimal points
    let b4 = 2.1782814781147811478114781147811478114781;
    println!("a3:{}",a4);
    println!("b3:{}",b4);

}