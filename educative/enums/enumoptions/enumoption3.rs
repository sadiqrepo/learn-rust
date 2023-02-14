// Index out of bounds exception

fn main(){
    let str = String::from("Educative");
    let index = 12;
    lookup(str, index);
}

fn lookup(s:String, i: usize){
    let mi = match s.chars().nth(i){
        Some(c) => c.to_string(),
        None => "No matched index found!".to_string(),
    };
    println!("{}",mi);
}