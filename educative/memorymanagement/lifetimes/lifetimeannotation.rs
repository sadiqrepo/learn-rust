#[allow(dead_code)]
struct Course {
    name:String,
    code: i32,
}
//fn fun_name<'a , 'b>(x: & 'a i32 , y: & 'b i32) 
//Multiple references have different lifetimes

//fn fun_name <'a>(x: & 'a i32 , y: & 'a i32) -> & 'a i32 
//Multiple references have the same lifetime
fn get_course <'a>(c1:&'a Course, c2:&'a Course)-> &'a Course {
    if c1.name == "Rust"{
        c1
    } else {
        c2
    }
}

fn main(){
    let c1 = Course {
        name:String::from("Rust"),
        code: 101,
    };

    let c2 = Course {
        name: String::from("Go"),
        code: 102,
    };

    get_course(&c1, &c2);
}