#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields

struct Car {
    color: String, 
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type

enum Transmission {
    Manual, 
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn car_quality (miles: u32) -> (Age, u32){
    //Declare & Initialize the return tuple value
    //For new car set the miles to 0.

    if miles == 0 {
        return (Age::New, miles);
    }

    return (Age::Used, miles);
    
}

fn car_factory (color:String, motor:Transmission, roof:bool, miles: u32) -> Car {

    if car_quality(miles).0 == Age::Used {
        if roof {
            println!("Prepare Used car: {:?}, {}, {}, {} miles",motor, color, roof, miles);
        } else {
            println!("Prepare Used car: {:?}, {}, {}, {} miles ",motor, color, roof, miles);
        }
    } else {
            if roof {
                println!("Build New car: {:?}, {}, {}, {} miles ",motor, color, roof, miles);
            } else {
                println!("Build New car: {:?}, {}, {}, {} miles ",motor, color, roof, miles);
            }
        
    }

    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}
fn main() {

    //Create car color array
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Declare the car type & initial values
    //let mut car: Car;
    //let mut engine = Transmission::Manual;

    // Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car_factory(String::from(colors[0]), Transmission::Manual, true, 0);
    //println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);


    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from(colors[2]), Transmission::SemiAuto, false, 100);
    //println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from(colors[3]), Transmission::Automatic, true, 200);
    //println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

}
