fn main() {

    //Declare a number variable as a 32-bit integer

    let number: u32 = 10;
    println!("The Integer value is {}.",number);

    //Declare floating point variables
    let float_one = 5.0;
    let float_two: f32 = 2.0;
    println!("float_one / float_two is {}", float_one / float_two);

    //Arithmetic operations
    println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

    // Interger and Floating point division
    println!("9 / 2 = {} but 9.0 /2.0 = {}", 9u32 / 2, 9.0 / 2.0);


    //Declare a variable to store result of 'greater than' test, Is 1 > 4? -- false
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);

    //Declare char data type
    let character_1 = 'S';
    let character_2 = 'f';

    //Compiler interprets a single item in quotations as the 'char' data type
    let smiley_face = '😃';

    // Compiler interprets a series of items in quotations as a'str' data type and creates "&str" reference
    let string_1 = "miley ";
    let string_2: &str = "ace";
    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);



}
