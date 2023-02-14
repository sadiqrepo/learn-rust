// Optional variable value

struct OptionalCourse {
    name:String,
    level: Option<String>,
    code: i32,
}

fn main(){

    let oc1 = OptionalCourse {
        name: String::from("Go"),
        level: Some(String::from("Beginner")),
        code: 104,
    };

    let oc2 = OptionalCourse {
        name: String::from("Rust"),
        level: None,
        code: 105,
    };

    println!("Name: {}; Level: {}; Code: {}",oc1.name, oc1.level.unwrap_or("Level".to_string()), oc1.code);
    println!("Name: {}; Level: {}; Code: {}",oc2.name, oc2.level.unwrap_or("No Level Defined!".to_string()), oc2.code);

}