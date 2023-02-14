fn main(){
    /**
      Operators are listed below in the order of their precedence from highest to lowest :

            Unary
                Logical/Bitwise NOT - !
                Derereference - *
                Borrow - &, &mut

            Binary
                Typecast - as
                Multiplication- *,Division - /, Remainder - %
                Addition -+, Subtraction - -
                Left Shift - <<, Right Shift - >>
                Bitwise AND - &
                Bitwise XOR - ^
                Bitwise OR - |
                Comparison - == != < > <= >=
                Logical AND - &&
                Logical OR - ||
                Range - start .. stop
                Assignment/Compound Assignment - = += -= *= /= %= &= |= ^= <<= >>=
     */

    println!("{}", 3 + 4 - 9 / 6 * 6 ^ 8 & 3);


   
        let a = 2;
        let b = 2;
        let c = i32::pow(a,3) + i32::pow(b, 3) + ( 3 * a * b * (a + b)) ; 
        println!("{}",c);  



}