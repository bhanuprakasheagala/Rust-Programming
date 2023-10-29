/*
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
Tuples have a fixed length: once declared, they cannot grow or shrink in size.

We create a tuple by writing a comma-separated list of values inside parentheses.
Each position in the tuple has a type, and the types of the different values in the tuple 
don’t have to be the same. We’ve added optional type annotations in this example:
*/

fn main(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //Access tuple elements using dot notation
    let mut val_at_index_zero = tup.0;
    val_at_index_zero += 1;
    let mut val_at_index_one = tup.1;
    val_at_index_one += 1.0;
    let mut val_at_index_two = tup.2;
    val_at_index_two += 1;

    println!("Elements: {} {} {} ", val_at_index_zero, val_at_index_one, val_at_index_two);
}