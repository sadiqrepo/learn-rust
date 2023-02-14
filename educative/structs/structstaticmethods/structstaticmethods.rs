struct Car {
    make: String,
    model:String,
    year: i32,
}


impl Car {

    fn car_details(mk:String, mdl:String, y:i32)->Car {
        Car {
            make: mk,
            model:mdl,
            year:y,
        }
    }

    fn display_car_details(&self){
        println!("Make: {}; Model: {}; Year: {}", self.make, self.model, self.year);
    }
}

fn main(){

    let c1 = Car::car_details("Tata".to_string(), "Nexon".to_string(), 2022);
    c1.display_car_details();
}