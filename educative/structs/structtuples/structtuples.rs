
struct FruitQuantity(String, i32);

fn main(){
    let fruit1 = FruitQuantity("Apple".to_string(), 240);
    println!("Fruit name: {}; Quantity: {}",fruit1.0, fruit1.1);

    let fruit2 = FruitQuantity("Mangoes".to_string(), 360);
    println!("Fruit name: {}; Quantity: {}",fruit2.0, fruit2.1);

}
