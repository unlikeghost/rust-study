// https://practice-rust.beatai.org/variables.html
// The code current code is to solve the problems that are shown on the website

fn main() {
    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7_1();
    problem_7_2();
    problem_8();
    problem_9();
}

/*
// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
 */
fn problem_1() {
    println!("Problem 1");
    let x: i32 = 5;
    let y: i32;

    assert_eq!(x, 5);
    println!("Success! \n");
}

/* 
Fill the blanks in the code to make it compile
fn main() {
    let __ __ = 1;
    __ += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}
 */
fn problem_2() {
    println!("Problem 2");
    let mut x = 1;
    x += 2;
   
    assert_eq!(x, 3);
    println!("Success! \n");
}

/*
Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y); 
}
*/

fn problem_3() {
    println!("Problem 3");
    let x: i32 = 10;
    let y: i32 = 5;

    {
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }

    println!("Outer scope value of x is {} and value of y is {}", x, y); 

    println!("Success! \n");
}

/*
// Fix the error with the use of define_x
fn main() {
    println!("{}, world", x); 
}

fn define_x() {
    let x = "hello";
}
*/

fn problem_4(){
    println!("Probelm 4");
    define_x();
    println!("Success! \n")
}
fn define_x() {
    let x = "hello";
    println!("{}, world", x);
}

/*
Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x = 42;
    println!("{}", x); // Prints "42".
}
*/
fn problem_5(){
    println!("Problem 5");
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x);

    println!("Success! \n");
}

/*

// Remove a line in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x; 
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}
*/
fn problem_6(){
    println!("Problem 6");
    
    let mut x: i32 = 1;
    x = 7;

    // // shadowing and re-binding
    // let x = x;
    x += 3;

    let y = 4;
    let y = "I can also be bound to text!";

    println!("Success! \n");
}

/*
Fix the warning below with :
🌟 Only one solution
🌟🌟 Two distinct solutions
Note: none of the solutions is to remove the line let x = 1

fn main() {
    let x = 1; 
}
*/
fn problem_7_1(){
    println!("Problem 7.1");
    let _x = 1;
    println!("Success! \n");
}

#[allow(unused_variables)]
fn problem_7_2(){
    println!("Problem 7.1");
    let x = 1;
    println!("Success! \n");
}

/*
// Fix the error below with least amount of modification
fn main() {
    let (x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
*/
fn problem_8(){
    println!("Problem 8");
    let (mut x, y) = (1, 2);
    x+=2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
    
    println!("Success! \n");
}

/*
fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], __);

    println!("Success!");
} 
*/
fn problem_9(){
    println!("Problem 9");
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
}