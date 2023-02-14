struct Circle {
    radius: f32,
}

struct Rectangle {
    length: f32,
    width: f32,
}

trait Area {
    fn shape_area(&self)-> f32;
}

impl Area for Circle {
    fn shape_area(&self)->f32 {
        3.14 * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn shape_area(&self)->f32 {
         self.length * self.width
    }
}

fn main(){

    let c = Circle {
        radius: 2.0,
    };

    let r = Rectangle {
        length: 4.0,
        width: 3.0,
    };

    println!("Area of Circle: {}", c.shape_area());
    println!("Area of Rectangle: {}", r.shape_area());
}