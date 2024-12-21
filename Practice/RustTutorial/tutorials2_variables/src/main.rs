fn main() {
    let mut x = 4; /*is only possible due to MUT 
                    making the let mutable 
                    and making it able to be changed*/ 
    println!("Hello, x is: {}", x);
    x = 5;
    //making a "isolated scope" / a method?
    {
        let x = x - 432;
        println!("x is now: {}", x)
    }
    println!("x is now: {}", x); 
    
    //changing the variable of y without making it mutable:
    let y = 6;
    println!("Hello, y is: {}", y);
    let y = 7;
    println!("Woah!, y is now: {} despite not being mutable!", y)
    //this was possible due to y being "set anew" with the let key.
    
}
