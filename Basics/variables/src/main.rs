fn main() {
    println!("Variables in Rust");
    
    let mut a = 5;
    println!("The value of a is: {a}");
    a = 6;
    println!("The value of a is: {a}");

    // Shadowing
    let x = 10;
    
    let x = x + 2;
    
    {
	let x = x * 2;
	println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");


    let spaces = "   ";  // string
    let spaces = spaces.len(); //using the first spaces, final value is int
    println!("Number of spaces typed is: {spaces}");
    
    /*

    The following way will give an error: expected `&str`, found `usize`
    let mut spaces = "   ";
    spaces = spaces.len();

    */
}
