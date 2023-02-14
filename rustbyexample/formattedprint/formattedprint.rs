#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main(){

    // In general, {} will automatically replace the arguements
    println!("{} days",31);

    // Positional arguements example
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");


    //Named arguements
    println!("{subject} {verb} {object}", object ="the lazy dog", subject="the quick brown fox", verb="jumps over");

    //Formatting using format characters
    println!("Base 10: {}",69420);
    println!("Base 2 (binary): {:b}",69420);
    println!("Base 8 (octal): {:o}",69420);
    println!("Base 16 (hexadecimal): {:x}",69420);
    println!("Base 16 (hexadecimal): {:X}",69420);


    //Right justify text
    println!("{number:>5}",number=1);

    //pad number with extra zeroes and left-adjust
    println!("{number:0<5}", number=1);

    //named arguements with a $
    println!("{number:>width$}", number=2, width=5);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    /* 
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);
    println!("This struct {:?} won't print...", Structure(3));
*/
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi: f64 = 3.141592;
    println!("{}",pi);

    // Debug Examples
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");


    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

   
    
}