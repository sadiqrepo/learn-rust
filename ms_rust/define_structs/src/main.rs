fn main() {

    //Classic struct with named fields
    struct Student { name: String, level: u8, remote: bool }

    //Tuple struct with data types
    struct Grades(char, char, char, char, f32);

    //Unit struct
    //struct Unit;

    let user_1 = Student { name: String::from("John snow"), level: 5, remote: true };
    let user_2 = Student { name: String::from("Dyson Tan"), remote: false, level: 2 };

    let mark_1 = Grades('A', 'B', 'A', 'C', 3.75);
    let mark_2 = Grades('B', 'C', 'A', 'C', 2.50);

    println!("{}. level: {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
                user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);

    println!("{}. level: {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
                user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);
}
