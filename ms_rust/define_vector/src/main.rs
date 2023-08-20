
fn define_vector() {

    let mut three_nums = vec![15, 20, 25];
    let zeroes = vec![0; 5];
    let mut fruit = Vec::new();
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");

    println!("Fruits Before removal: {:?}", fruit);

    println!("Pop off: {:?}",fruit.pop());
    println!("Fruits after removal: {:?}", fruit);

    println!("Vector 1: {:?}, three_nums[1]: {}", three_nums, three_nums[1]);
    println!("Vector 2: {:?}", zeroes);

}

fn main() {
    define_vector();
}
