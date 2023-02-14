fn reverse(my_vec: &mut Vec<i64>)-> &mut Vec<i64>{
    // your code goes here!
    my_vec.reverse();
    my_vec
 } 

 fn main(){
    let mut my_vec: Vec<i64> = vec![1,2,3,4,5];
    let result = reverse(&mut my_vec);
    println!("{:?}",result);
   
   }