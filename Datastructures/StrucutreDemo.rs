// 1. Example: Basics of struct

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main(){
    println!("What is your first name?");
    let first_name = read_string();
    println!("What is your last name?");
    let last_name = read_string();
    println!("What is your Age?");
    let age = read_number();

    let person = Person {
        first_name: first_name,
        last_name: last_name,
        age: age,
    };

    print_person(&person);
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    let cleaned_input = input.trim().to_string();

    cleaned_input
}

fn read_number() -> u8 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read input!");
    let input: u8 = input.trim().parse().expect("Not a positive number");

    input
}

/*
Note that we lend our person to the print_person function with the & operator. The & must be added to the signature of the print_person
function which os taking a &Person as input, as well as the calling function that is passing &person.
*/
fn print_person(person: &Person){
    println!("Hello {} {}", person.first_name, person.last_name);
    println!("Your Age: {}", person.age);
}
