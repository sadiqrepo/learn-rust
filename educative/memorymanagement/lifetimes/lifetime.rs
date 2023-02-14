/*
Lifetime references the scope that a reference is valid for.
Referencing a resourse that is deallocated is called - Dangling pointer or use after free
 */

 struct Course {
    name: String,
    id: i32,
 }

 fn main(){
    let c1:&Course {
        let c2: Course = Course {
            name:String::from("Rust"),
            id: 101,
        };
    }

    // Allocated memory reference to a location that is dropped
    //c1 = &c2;
 }