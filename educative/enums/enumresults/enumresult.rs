fn main(){
    println!("\n{:?}", file_found(true));
    println!("{:?}", file_found(false));

    println!("\n{:?}",divisible_by_3(6));
    println!("{:?}",divisible_by_3(7));

    let check1 = divisible_by_3(6);
    if check1.is_ok(){
        println!("\nGiven number is divisible by 3");
    } else {
        println!("Given number is not divisible by 3");
    }

    let check2 = divisible_by_3(7);
    if check1.is_err(){
        println!("Given number is not divisible by 3");
    } else {
        println!("Given number is divisible by 3");
    }

    assert_eq!(check1.is_ok(),true);
    assert_ne!(check2.is_err(),false);
}

fn file_found(val:bool)-> Result<i32, bool>{
    if val {
        Ok(200)
    }else {
        Err(false)
    }
}

fn divisible_by_3(num:i32)-> Result<String, String>{
    if num % 3== 0 {
        Ok("Given number is divisible by 3".to_string())
    } else {
        Err("Given number is not divisible by 3".to_string())
    }
}