fn main(){
    println!("What is your name?");
    let input = read_string(); // User defined function
    let clean_input = read_clean_string(); // same as read_string but trim white spaces
    println!("Your name is: {}", input);
    println!("Your name is: {}", clean_input);
}

// read_string returns a String datatype. A String is a convenient wrapper around the `str` data type.
// Unlike `str` it can be mutated which is exactly what we need in our function.

fn read_string() -> String {
    // Before we can read anything, we need to create a place to store the user's input.
    // The String::new() function creates a new String in memory. We use the mut keyword to indicate that we'll be changing
    // the String later on.
    // The :: separator is used to indicate that we'll be calling a function that belongs to the preceding item.
    // In this case we call the new() function that belongs to the String datatype.
    let mut input = String::new();

    /*
    The dot . chains together a sequence of operations. In our case:
    std::io::stdin()   To get access to the keyboard
    read_line(&mut input)   To read a string of text and store the result in input
    expect("can not read user input")   To write an error in case something prevents us from reading (no keyboard??)
    */
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    // The &mut keyword indicates that the read_line function can modify the contents of the input variable.

    /*
        Finally the contents of input is returned at the end of the read_string function. Notice that there is no ; at the
        end of the input statement. Omitting the ; at the end of input tells Rust to return the content of that
        variable as a result of the function. It is shorthand for `return input;` Which would essentially do the same.
    */
    input
}

/*
read_clean_string, that reads the input, cleans it with trim(), and then returns it.
You need to call the to_string() method on the cleaned result to convert it to a String.
*/

fn read_clean_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input = input.trim().to_string();

    input
}