fn main(){
    println!("Right angled triangle when n = 5 ");
    test(5);
    println!("Right angled triangle when n = 6 ");
    test(6);
}

fn test(n:i32) {
    // Write code here!
    for i in 1..n + 1 { //outer loop
            for _j in 1..i +1 { // inner loop
                print!("{}", "&");
            }
            println!()
    }
   }