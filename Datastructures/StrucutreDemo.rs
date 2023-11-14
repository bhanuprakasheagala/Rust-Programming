// 1. Example: Basics of struct

struct Person {
    first_name: String,
    last_name: String,
}

fn main(){
    println!("What is your first name?");
    let first_name = read_string();
    println!("What is your last name?");
    let last_name = read_string();

    let person = Person {
        first_name: first_name,
        last_name: last_name,
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

fn print_person(person: &Person){
    println!("Hello {} {}", person.first_name, person.last_name);
}
