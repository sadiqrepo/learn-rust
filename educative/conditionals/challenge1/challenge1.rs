fn main(){
    let _a = 5;
    test(_a)
}

fn test(_a:i32) { 
    // Write code here
    if _a % 2 != 0 {
       println!("Number {} is odd",_a)
    } else {
       println!("Number {} is even",_a)
    }
 }