fn test(my_str:String)-> String {
    let mut my_updated_string = "".to_string(); 
    for word in my_str.split_whitespace(){
         if word.starts_with("c"){
             my_updated_string.push_str(word);
             my_updated_string.push(' ');
         }
        }
    my_updated_string.pop();
    my_updated_string
}
fn main(){
    let  my_str= "This is a comprehensive course in Rust programming language on Educative. Read it with full concentration to grasp the content of the course";
    println!("Original String: {}", my_str);
    let updated_string = test(my_str.to_string());
    println!("Updated String: {}", updated_string);
}