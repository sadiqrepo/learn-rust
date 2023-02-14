fn test_divisibility_by_3_4(a:i32)->i32{
    //check if number is divisible by 3 and 4 
    if a % 3 == 0 && a % 4 == 0{
        0
    }
    //check if number is divisible by 3 and not by 4 
    else if a % 3 == 0 && a % 4 != 0 {
        1
    }
    //check if number is divisible not by 3 but 4 
    else if a % 3 != 0 && a % 4 == 0 {
        2
    }
    //check if neither divisible by 3 nor 4
    else {
        -1
    }
}
fn main(){
    println!(" Number = 12 : {}", test_divisibility_by_3_4(12));
    println!(" Number = 9  : {}", test_divisibility_by_3_4(9));
    println!(" Number = 8  : {}", test_divisibility_by_3_4(8));
    println!(" Number = 23 : {}", test_divisibility_by_3_4(23));
}