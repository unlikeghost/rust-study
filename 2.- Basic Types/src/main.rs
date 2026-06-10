// https://practice-rust.beatai.org/variables.html
// The code current code is to solve the problems that are shown on the website


fn main() {
    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();
    problem_8_1();
    problem_8_2();
    problem_9();
    problem_10();
    problem_11();
    problem_12();
    problem_13();
    problem_14();
    problem_15();
    problem_16();
    problem_17();
    problem_18_1();
    problem_18_2();
    problem_19();
    problem_20();
    problem_23();
}

#[allow(unused_variables)]
fn problem_1(){
    /*
    // Remove something to make it work
    fn main() {
        let x: i32 = 5;
        let mut y: u32 = 5;
    
        y = x;
        
        let z = 10; // Type of z ? 
    
        println!("Success!");
    }
    */
    println!("Problem 1");

    let x: i32 = 5;
    let mut y: u32 = 5;
    
    let z: i8 = 10; // Type of z ? 

    println!("Success! \n");
}


#[allow(unused_variables)]
fn problem_2(){
    /*
    // Fill the blank
    fn main() {
        let v: u16 = 38_u8 as __;
    
        println!("Success!");
    }
    */
    println!("Probelm 2");
    
    let v: u16 = 38_u8 as u16;
    
    println!("Success! \n");
}

fn problem_3(){
    /*
    // Modify `assert_eq!` to make it work
    fn main() {
        let x = 5;
        assert_eq!("u32".to_string(), type_of(&x));
    
        println!("Success!");
    }
    
    // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
    */
    println!("Problem 3");
    
    let x: i32 = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success! \n");
}
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


fn problem_4(){
    /*
     // Fill the blanks to make it work
     fn main() {
         assert_eq!(i8::MAX, __); 
         assert_eq!(u8::MAX, __); 
     
         println!("Success!");
     }
     */
     println!("Problem 4");
     assert_eq!(i8::MAX, 127); 
     assert_eq!(u8::MAX, 255); 
 
     println!("Success! \n");
}

fn problem_5(){
    /*
    // Fix errors and panics to make it work
    fn main() {
       let v1 = 251_u8 + 8;
       let v2 = i8::checked_add(251, 8).unwrap();
       println!("{},{}",v1,v2);
    }
    */
    println!("Problem 5");

    let v1: u16 = 251_u16 + 8_u16;

    let v2: u16 = u16::checked_add(251, 8).unwrap();
    
    println!("{},{}",v1,v2);
    println!("Success! \n");
}

fn problem_6(){
    /*
    
    // Modify `assert!` to make it work
    fn main() {
        let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
        assert!(v == 1579);
    
        println!("Success!");
    }
    */
    println!("Problem 6");

    let v: u32 = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
    assert!(v == 1597);
    
    println!("Success! \n");
}

#[allow(unused_variables)]
fn problem_7(){
    /*
    // Fill the blank to make it work
    fn main() {
        let x = 1_000.000_1; // ?
        let y: f32 = 0.12; // f32
        let z = 0.01_f64; // f64
    
        assert_eq!(type_of(&x), "__".to_string());
        println!("Success!");
    }
    
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
    */
    println!("Problem 7");
    
    let x: f64 = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z: f64 = 0.01_f64; // f64
    
    assert_eq!(type_of(&x), "f64".to_string());

    println!("Success! \n");
}

fn problem_8_1(){
    /*
    fn main() {
        Make it work in two distinct ways
        assert!(0.1+0.2==0.3);
    
        println!("Success!");
    }
    */
    println!("Problem 8.1");

    assert!(0.1_f32+0.2_f32==0.3_f32);
    
    println!("Sucess! \n");
}

fn problem_8_2(){
    /*
    fn main() {
        Make it work in two distinct ways
        assert!(0.1+0.2==0.3);
    
        println!("Success!");
    }
    */
    println!("Problem 8.2");

    assert!(0.1 as f32 + 0.2 as f32 ==0.3 as f32);
    
    println!("Sucess! \n");
}

#[allow(unused_variables)]
fn problem_9(){
    /*
    🌟🌟 Two goals: 1. Modify assert! to make it work 2. Make println! output list of numbers between 97 and 122
    fn main() {
        let mut sum = 0;
        for i in -3..2 {
            sum += i
        }
    
        assert!(sum == -3);
    
        for c in 'a'..='z' {
            println!("{}",c);
        }
    }
     */
     println!("Problem 9");
     
     let mut sum = 0;
     for i in -3..2 {
         sum += i
     }

     assert!(sum == -5);

     for c in 'a'..='z' {
         println!("{}",c as u8);
     }
     
     println!("Success! \n");
 
}

use std::ops::{Range, RangeInclusive};

fn problem_10(){
    /*
    
    // Fill the blanks
    use std::ops::{Range, RangeInclusive};
    fn main() {
        assert_eq!((1..__), Range{ start: 1, end: 5 });
        assert_eq!((1..__), RangeInclusive::new(1, 5));
    
        println!("Success!");
    }
    */
    println!("Problem 10");

    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    
    println!("Success! \n");
     
}


fn problem_11(){
    /*
    
    // Fill the blanks and fix the errors
    fn main() {
        // Integer addition
        assert!(1u32 + 2 == __);
    
        // Integer subtraction
        assert!(1i32 - 2 == __);
        assert!(1u8 - 2 == -1); 
        
        assert!(3 * 50 == __);
    
        assert!(9.6 / 3.2 == 3.0); // error ! make it work
    
        assert!(24 % 5 == __);
        // Short-circuiting boolean logic
        assert!(true && false == __);
        assert!(true || false == __);
        assert!(!true == __);
    
        // Bitwise operations
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    }
     */

    println!("Problem 11");
    assert!(1u32 + 2u32 == 3u32);
    
    // Integer subtraction
    assert!(1i32 - 2i32 == -1i32);
    assert!(1i8 - 2i8 == -1i8); 

    assert!(3 * 50 == 150);

    assert!(9.6f32 / 3.2f32 == 3.0f32); // error ! make it work

    assert!(24 % 5 == 4);
    
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("Success! \n");
}

use std::mem::size_of_val;

fn problem_12(){
    /*
    // Make it work
    use std::mem::size_of_val;
    fn main() {
        let c1 = 'a';
        assert_eq!(size_of_val(&c1),1); 
    
        let c2 = '中';
        assert_eq!(size_of_val(&c2),3); 
    
        println!("Success!");
    } 
    */
    println!("Problem 12");
    
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1),4); 

    let c2: char = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success! \n");
    
}

fn problem_13(){
    /*
    
    // Make it work
    fn main() {
        let c1 = "中";
        print_char(c1);
    } 
    
    fn print_char(c : char) {
        println!("{}", c);
    }
    */
    println!("Problem 13");

    let c1 = '中';
    print_char(c1);
    
    println!("Success! \n");
}

fn print_char(c : char) {
    println!("{}", c);
}


fn problem_14(){
    /*
    // Make println! work
    fn main() {
        let _f: bool = false;
    
        let t = true;
        if !t {
            println!("Success!");
        }
    } 
    */

    println!("Problem 14");

    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success! \n");
    }
    
}

fn problem_15(){
    /*
    // Make it work
    fn main() {
        let f = true;
        let t = true && false;
        assert_eq!(t, f);
    
        println!("Success!");
    }
    */

    println!("Problem 15");

    let f = true;
    let t = true || false;
    assert_eq!(t, f);
    
    println!("Success! \n");
}

#[allow(unused_variables)]
fn problem_16(){
    /*
    // Make it work, don't modify `implicitly_ret_unit` !
    fn main() {
        let _v: () = ();
    
        let v = (2, 3);
        assert_eq!(v, implicitly_ret_unit());
    
        println!("Success!");
    }
    
    fn implicitly_ret_unit() {
        println!("I will return a ()");
    }
    
    // Don't use this one
    fn explicitly_ret_unit() -> () {
        println!("I will return a ()");
    }
    */

    println!("Problem 16");

    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());
    
    println!("Success! \n");
}

fn implicitly_ret_unit() -> () {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}


fn problem_17(){
    /*
    use std::mem::size_of_val;
    fn main() {
        let unit: () = ();
        assert!(size_of_val(&unit) == 4);
    
        println!("Success!");
    }
    */

    println!("Problem 17");

    let unit: () = ();
    assert!(size_of_val(&unit) == 0);
    
    println!("Success! \n");
}


fn problem_18_1(){
    /*
    // Make it work with two ways

    fn main() {
       let v = {
           let mut x = 1;
           x += 2
       };
    
       assert_eq!(v, 3);
    
       println!("Success!");
    }
    */

    println!("Problem 18.1");

    let v = {
        let x = 1;
        x + 2
    };
 
    assert_eq!(v, 3);
    
    println!("Success! \n");
}

fn problem_18_2(){
    /*
    // Make it work with two ways

    fn main() {
       let v = {
           let mut x = 1;
           x += 2
       };
    
       assert_eq!(v, 3);
    
       println!("Success!");
    }
    */

    println!("Problem 18.2");

    let v: () = {
        let mut x = 1;
        x+=2
    };
 
    assert_eq!(v, ());
    
    println!("Success! \n");
}


fn problem_19(){
    /*
    fn main() {
       let v = (let x = 3);
    
       assert!(v == 3);
    
       println!("Success!");
    }
    */

    println!("Problem 19");

    let v = {
        let x: u8 = 3;
        x
    };
 
    assert!(v == 3);
     
    println!("Success! \n");
}


fn problem_20(){
    /*
    fn main() {
        let s = sum(1 , 2);
        assert_eq!(s, 3);
    
        println!("Success!");
    }
    
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    */

    println!("Problem 20");

    let s: i32 = sum(1 , 2);
    assert_eq!(s, 3);
     
    println!("Success! \n");
}
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn problem_21(){
    /*
    fn main() {
        // Don't modify the following two lines!
        let (x, y) = (1, 2);
        let s = sum(x, y);
    
        assert_eq!(s, 3);
    
        println!("Success!");
    }
    
    fn sum(x, y: i32) {
        x + y;
    }
    */
    println!("Problem 21");

    let (x, y): (i32, i32) = (1, 2);
    let s: i32 = sum(x, y);

    assert_eq!(s, 3);
    
    println!("Success! \n");
}


fn problem_22(){
    /*
    fn main() {
        print();
     }
     
     // Replace i32 with another type
     fn print() -> i32 {
        println!("Success!");
     }
    */
    println!("Problem 22");
    print();

}
fn print() -> () {
    println!("Success! \n");
}


fn problem_23(){
    /*
    // DON'T let `println!` work
    fn main() {
        never_return();
    
        println!("Failed!");
    }
    
    fn never_return() -> ! {
        // Implement this function, don't modify the fn signatures
        
    }
    */
    println!("Problem 23");
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!("Succes! \n")
}