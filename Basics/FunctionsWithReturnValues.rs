/*
In Rust, the return value of the function is synonymous with the value of the final expression
in the block of the body of a function. You can return early from a function by using
the return keyword and specifying a value, but most functions return the last expression implicitly.
Here’s an example of a function that returns a value:
*/

/*
fn five() -> i32 {
    5
}

fn main(){
    let x = five();
    println!("The value of x is {x}");
}
*/

/*In the above five function:
There are no function calls, macros, or even let statements in the five function—just
the number 5 by itself. That’s a perfectly valid function in Rust.
Note that the function’s return type is specified too, as -> i32.

first, the line let x = five(); shows that we’re using the return value of a
function to initialize a variable.
Because the function five returns a 5, that line is the same as the following: let x = 5;

Second, the five function has no parameters and defines the type of the return value,
but the body of the function is a lonely 5 with no semicolon
because it’s an expression whose value we want to return.
*/

fn main(){
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32{
    x + 1
}

/*
But if we place a semicolon at the end of the line containing x + 1,
changing it from an expression to a statement, we’ll get an error.

The definition of the function plus_one says that it will return an i32, but
statements don’t evaluate to a value, which is expressed by (), the unit type.
Therefore, nothing is returned, which contradicts the function definition and results in an error. 
*/