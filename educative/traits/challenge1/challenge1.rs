struct Car{
    owners_age:i32,
}

struct MotorBike{
    owners_age:i32,
}

trait Drive {
    fn can_drive(&self)->i32;
}

impl Drive for Car{

    fn can_drive(&self)->i32{
        if self.owners_age >=18 {
            1
        } else {
            0
        }
    }
}

impl Drive for MotorBike{

    fn can_drive(&self)->i32{
        if self.owners_age >=14 {
            1
        } else {
            0
        }
    }
}

fn main(){
    let mut c = Car {
        owners_age: 16,
    };
    println!("\nCan owners_age: {} drive the car? {}",c.owners_age, c.can_drive());
    c.owners_age = 20;
    println!("Can owners_age: {} drive the car? {}",c.owners_age, c.can_drive());

    let mut m = MotorBike {
        owners_age: 10,
    };
    println!("\nCan owners_age: {} drive the MotorBike? {}",m.owners_age, m.can_drive());
    m.owners_age = 20;
    println!("Can owners_age: {} drive the MotorBike? {}",m.owners_age, m.can_drive());
}