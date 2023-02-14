/* Borrowing
a. Shared borrow - Assignee variable has ownership & assigned variable can only read the value
b. Mutable borrow - Assignee variable has ownership & assigned variable can share & mutate the value
*/
/*Rule #1. You can do one mutable borrow or any number of immutable borrows within same scope
  You cannot do both shared & mutable borrow inside same scope*/

fn main(){
    let mut a = 1;
    println!("Variable a: {}",a );

    let b = 2;
    println!("Variable b: {}", b);

    {
        let r1 = &a;
        println!("Variable r1 references a in inner scope SHARED BORROW(a): {}",r1);

        let r2 = &a;
        println!("Variable r2 references a in inner scope inside SHARED BORROW(a): {}",r2);
        println!("r1:{}\nr2:{}",r1, r2);

    }

    let r3 = &mut a;
    *r3 = 3;
    println!("Variable r3 references a in Outer scope inside MUTABLE BORROW(a) & dereferenced it & updated value: {}",r3);

    let r4 = &b;
    println!("Variable r4 references b in Outer scope inside SHARED BORROW(a): {}",r4);

    let r5 = &b;
    println!("Variable r5 references b in Outer scope inside SHARED BORROW(a): {}",r5);
    println!("r3:{}\nr4:{}\nr5:{}",r3, r4, r5);

    /* Rule #2: References must always be valid.
    Cannot reference a value that is moved, i.e., a non-primitive data type. */

    let t1 = String::from("Rust Programming");
    println!("Value of t1: {}",t1);
    let t2 = t1;

    println!("Value of t1 is moved to t2. \n t2: {}",t2);
    println!("Now t1 is invalid & accessing it will give error");

    //t1 value is moved and t3 cannot access it, hence below code will result it ERROR
    //let t3 = &t1;
    //println!("Variable t3 trying to access value of t1: {}",t3);

    let x, y, z 


}

