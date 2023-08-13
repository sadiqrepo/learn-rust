fn main() {
    
    //Declare first variable binding with name "shadown_num"
    let shadow_num = 5;
    println!("Shadow number after First binding is {}.", shadow_num);

    //Declare second variable binding, shadows existing variable "shadow_num"
    let shadow_num = shadow_num + 5;
    println!("Shadow number after Second binding is {}.", shadow_num);

    //Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2;
    println!("Shadow number after Third binding is {}.", shadow_num);
}
