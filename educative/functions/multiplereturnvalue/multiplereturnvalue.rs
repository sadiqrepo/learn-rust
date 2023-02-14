fn main(){
    let length = 10;
    let width = 5;
    println!("Rectangle length: {}", length);
    println!("Rectangle width: {}", width);
    let (area, perimeter) = calculate_area(length, width);
    println!("Area of Rectangle: {}", area);
    println!("Perimeter of Rectangle: {}", perimeter);

}

fn calculate_area(l:i32, w:i32)-> (i32, i32){
    let area = l * w;
    let perimeter = 2 * (l + w);
    (area, perimeter)
}