// https://practice-rust.beatai.org/ownership/ownership.html
// The code current code is to solve the problems that are shown on the website

fn main(){
    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();
    problem_8();
    problem_9();
}

fn problem_1(){
    /*
    fn main() {
        // Use as many approaches as you can to make it work
        let x = String::from("Hello world");
        let y = x;
        println!("{}, {}",x, y);
    }
    */
    println!("Problem 1");

    let x: String = String::from("Hello world");
    let y: String = x.clone();
    println!("{}, {}",x, y);
    
    println!("Success! \n");
}


fn problem_2(){
    /*
    // Don't modify code in main!
    fn main() {
        let s1 = String::from("Hello world");
        let s2 = take_ownership(s1);
    
        println!("{}", s2);
    }
    
    // Only modify the code below!
    fn take_ownership(s: String) {
        println!("{}", s);
    }
    */
    println!("Problem 2");

    let s1: String = String::from("Hello world");
    let s2: String = take_ownership(s1);

    println!("{}", s2);
    
    println!("Success! \n");
}
fn take_ownership(s: String) -> String{
    println!("{}", s);
    s
}


fn problem_3(){
    /*
    fn main() {
        let s = give_ownership();
        println!("{}", s);
    }
    
    // Only modify the code below!
    fn give_ownership() -> String {
        let s = String::from("Hello world");
        // Convert String to Vec
        let _s = s.into_bytes();
        s
    }
    */
    println!("Problem 3");

    let s = give_ownership();
    println!("{}", s);
    
    println!("Success! \n");
}
fn give_ownership() -> String {
    let s: String = String::from("Hello world");
    // Convert String to Vec
    let _s: Vec<u8> = s.clone().into_bytes(); // this consume the ownership of the variable
    // let _s2 = s.as_bytes() // this borrows the ownership!
    s
}


fn problem_4(){
    /*
    // Fix the error without removing any code
    fn main() {
        let s = String::from("Hello World");
    
        print_str(s);
    
        println!("{}", s);
    }
    
    fn print_str(s: String)  {
        println!("{}",s)
    }
    */
    println!("Problem 4");

    let s = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
    
    println!("Success! \n");
}
fn print_str(s: String)  {
    println!("{}",s)
}


fn problem_5(){
    /*
    // Don't use clone ,use copy instead
    fn main() {
        let x = (1, 2, (), "hello".to_string());
        let y = x.clone();
        println!("{:?}, {:?}", x, y);
    }
    */
    println!("Problem 5");

    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    let y: (i32, i32, (), &str) = x;
    println!("{:?}, {:?}", x, y);
    
    println!("Success! \n");
}


fn problem_6(){
    /*
    // make the necessary variable mutable
    fn main() {
        let s = String::from("Hello ");
        
        let s1 = s;
    
        s1.push_str("World!");
    
        println!("Success!");
    }
    */
    println!("Problem 6");

    let s: String = String::from("Hello ");
    
    let mut s1: String = s; // we are transferring the ownership however still immutable that's why we have to add mut 

    s1.push_str("World!");
    
    println!("Success! \n");
}

fn problem_7(){
    /*
     fn main() {
         let x = Box::new(5);
         
         let ...      // update this line, don't change other lines!
         
         *y = 4;
         
         assert_eq!(*x, 5);
     
         println!("Success!");
     }
    */
    println!("Problem 7");

    let x: Box<i32> = Box::new(5); // this is a pointer!
    
    let mut y: Box<i32> = Box::new(1);
    
    *y = 4;
    
    assert_eq!(*x, 5); // we use * in order to access values on pointers, if we don't use it, it will return an memory address 
    
    println!("Success! \n");
}


fn problem_8(){
    /*
    fn main() {
       let t = (String::from("hello"), String::from("world"));
    
       let _s = t.0;
    
       // Modify this line only, don't use `_s`
       println!("{:?}", t);
    }
    */
    println!("Problem 8");

    let t: (String, String) = (
        String::from("hello"), String::from("world")
    );
 
    let _s: String = t.0;
 
    println!("{:?}", t.1);

    println!("Success! \n");
}


fn problem_9(){
    /*
    fn main() {
       let t = (String::from("hello"), String::from("world"));
    
        // Fill the blanks
        let (__, __) = __;
    
        println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
    }
    */
    println!("Problem 9");

    let t: (String, String) = (String::from("hello"), String::from("world"));
 
     // Fill the blanks
     let (s1, s2): (String, String) = t.clone();
 
     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")

    println!("Success! \n");
}