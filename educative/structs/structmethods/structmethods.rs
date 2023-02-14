
struct Car {
    name:String,
    make:String,
    year:i32,
}

impl Car {
    fn name_year(&self)-> String {
        format!("{} {}", self.name, self.year)
    }
}

fn main(){

    let car1 = Car {
        name:String::from("Nexon"),
        make:String::from("Tata"),
        year:2022,
    };

    println!("This is {} {}",car1.make, car1.name_year());
}