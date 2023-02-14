
fn display_my_course1(course_taken1:&str){
    println!("\nCourse taken: {}",course_taken1);
}

fn display_my_course2(course_taken2:String){
    println!("\nCourse taken: {}",course_taken2);
}


fn main(){
    
    //Define a growable string variable
    let str = String::from("Rust Programming language");
    println!("This is a beginner course in {}",str);

    // Capacity in Bytes
    println!("Capacity {}, Length {}",str.capacity(),str.len());

    // Finding a substring
    let sub_str = String::from("Rust");
    println!("\n{} is a substring of {}: {}",sub_str,str, str.contains("Rust"));

    // Replace a substring
    let replace_from = "Rust";
    let replace_to = "Java";
    let result = str.replace(replace_from,replace_to);
    println!("\n{} now becomes {}",str, result);

    // Trim a string
    let string = "   Rust     Programming     ".to_string();
    let trim_string = string.trim(); 
    println!("Trimmed_string : {}", trim_string);
    println!();

    // String tokenizing on whitespace
    for token in str.split_whitespace() {
        println!("{}",token)
    }
    println!();

    //String tokenizing on custom character
    // define a String object
    let string1 = String::from("Educative, course on, Rust, Programming");  
    // split on token
    for token in string1.split(","){
        println!("{}", token);
    }
    println!();

    //Iterating over string object
    for token in string1.chars(){
        print!("{}", token);
    }
    println!();

    //Push a single character
    let mut string2 = String::from("Rus");
    string2.push('t');
    println!("\nThis is a beginner's course in {}",string2);

    //Push a string
    string2.push_str(" programming");
    println!("\nThis is a beginner's course in {}",string2);

    //String concatenation
    let string3 = " language".to_string();
    let result1 = string2 + &string3;
    println!("\n{}",result1);

    //Format macro
    let course = "Rust".to_string();
   let _course_type = "beginner course".to_string();
    let _result2 = format!("{} {}", course, _course_type);
    let _result2 = format!("{1} {0}", course, _course_type);
    println!("{}",_result2);

    //Slicing a string
    let string4 = "Rust Programming".to_string();
    let slice1 = &string4[5..12];
    println!("\nSlice: {}", slice1);

    // Passing primitive string(String Literal)
    let string5:&str = "Rust Programming";
    display_my_course1(string5);
    println!("Course offered: {}",string5);

    // Passing growable string (String Object)
    let string6:String = String::from("Rust Programming");
    display_my_course2(string6);


}
