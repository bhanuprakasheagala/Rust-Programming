// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of tuple of variables
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

fn main(){
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, 0.1f32, 0.2f64,'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), ('b', -1i32), 0.4f32);

    // Tuples are printable. But long Tuples (more than 12 elements) cannot be printed.
    println!("tuple of tuples {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parantheses
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructed to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a,b,c,d) = tuple;
    println!("{:?} {:?} {:?} {:?}", a,b,c,d);

}