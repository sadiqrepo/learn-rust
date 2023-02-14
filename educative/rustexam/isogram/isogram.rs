// Incorrect code

fn determine_isogram(word: &str) -> i32 {
    // Your code goes here!
    let seen = [false; 26];
    let mut result: i32 = 0;


    for c in s.chars() {
        let index = match c.to_ascii_lowercase() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            // and so on for the other letters of the alphabet
            _ => continue,
        };

        if seen[index] {
         seen[index] = false;
        } else {
            seen[index] = true;
        }
    }

    if seen {
      result = 0;
    } else {
      result = 1;
    }

    result
}
 

 
 fn main(){
    let str1:&str = "Isograms";
    println!("Is Isogram? {}",determine_isogram(str1));
 }