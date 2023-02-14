

// Return value is none
fn main(){
    println!("{:?}", learn_lang("Rust"));
    println!("{:?}", learn_lang("Go"));
}

fn learn_lang(str:&str)-> Option<bool> {
    if str == "Rust"{
        Some(true)
    }else {
        None
    }
}