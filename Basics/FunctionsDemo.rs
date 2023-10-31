/*
Rust code uses snake case as the conventional style for function and variable names, in which all 
letters are lowercase and underscores separate words.Here’s a program that contains an example 
function definition:
*/

/***
fn main(){
    println!("Hello, world!");

    another_function();
}
fn another_function(){
    println!("Another function");
} ***/

/*
In the above example:
Note that we defined another_function after the main function in the source code;
we could have defined it before as well. Rust doesn’t care where you define your functions,
only that they’re defined somewhere in a scope that can be seen by the caller.
*/

// Parameters
/*
In function signatures, you must declare the type of each parameter.
This is a deliberate decision in Rust’s design: requiring type annotations in
function definitions means the compiler almost never needs you to use them elsewhere
in the code to figure out what type you mean.
The compiler is also able to give more helpful error messages if it knows what types the function expects.
*/
fn main() {
    another_function(5);
    multiple_parameters(5, 'h');
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn multiple_parameters(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

