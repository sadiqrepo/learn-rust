
struct CourseDetails{
    name: String,
    level: String,
    code: i32,
}

fn display_my_course_info(c:CourseDetails){
    println!("\nCourse_Name: {}, Course_Level: {}, Course_Code: {}", c.name, c.level, c.code);
}

fn display_course_chosen(a:CourseDetails, b:CourseDetails)-> CourseDetails {
    println!("Choosing your course activated!!");
    if a.name == "Rust" {
        return a;
    }else {
        return b;
    }
}

fn main(){

    let mut course1 = CourseDetails {
        name: String::from("Java"),
        level: String::from("beginner"),
        code: 130,
    };

    let course2 = CourseDetails {
        name: String::from("Go"),
        level: String::from("beginner"),
        code: 104,
    };

    let course3 = CourseDetails {
        name: String::from("Rust"),
        level: String::from("beginner"),
        code: 117,
    }; 

    let course4 = CourseDetails {
        name: String::from("Python"),
        level: String::from("beginner"),
        code: 127,
    }; 
    //access struct
    println!("\nName: {}, Level: {}, Code: {}", course1.name, course1.level, course1.code);
    println!("Name: {}, Level: {}, Code: {}", course2.name, course2.level, course2.code);

    //Update
    course1.name = "Rust".to_string();
    course1.level = "Intermediate".to_string();
    course1.code = 128;
    println!("\nCourse 1 details updated!");
    println!("Name: {}, Level: {}, Code: {}", course1.name, course1.level, course1.code);

    //Passing structs to function
    display_my_course_info(course1);
    display_my_course_info(course2);

    //Returning struct from a function //  Below code is not working
    let chosen_course = display_course_chosen(course3, course4);
    println!("\nI choose to learn {} {} course & course code is {}", chosen_course.name, chosen_course.level, chosen_course.code);
    
}