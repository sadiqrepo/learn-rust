fn my_area(b: f32 , h: f32 ){ // root function
    let result:f32 = b * h * 0.5; // compute area of triangle
    println!("{}",result);
 }
 // declare a module
 mod shapes {
   // function within outer module
   pub fn triangle_area(x : i32 , y : i32) {
    super :: my_area ( x as f32 , y as f32); // invoke the root function
   }
 }

 use shapes::*;
 fn main(){
    print!("Area of triangle with width = 3 and height = 4 : ");
    triangle_area(3, 4);
 
    print!("Area of triangle with width = 9 and height = 4 : ");
    triangle_area(9, 4);
 }