// Adding an associated function:
// For String type there is an associated function new(), that we use to construct a new String.
// We can do the same for type Person type.

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

// The `impl Person { }` block is used to define associated functions, or methods that belong to the Person type.
impl Person {
    fn new(first_name: String, last_name: String, age: u8) -> Person {
        Person {
            first_name,
            last_name,
            age,
        }
    }
}

fn main(){
    println!("What is your first name?");
    let first_name = read_string();
    println!("What is your last name?");
    let last_name = read_string();
    println!("What is your Age?");
    let age = read_number();

    // We've added a new() function that takes three input parameters: first_name, last_name, age and returns a new Person object.
    let person = Person::new(first_name, last_name, age); // User defined associated function
    print_person(&person);
}
fn read_string() -> String {
    let mut input = String::new(); // Builting associated function
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can't read input!");
    input = input.trim().to_string(); // Trim whitespaces at the ends

    input
}

fn read_number() -> u8 {
    let mut num = String::new();
    std::io::stdin()
        .read_line(&mut num)
        .expect("Failed to read input!");
    let num: u8 = num.trim().parse().expect("\nNot a positive number!!\n");
    num
}

fn print_person(person: &Person) {
    println!("Hello {} {}", person.first_name, person.last_name);
    println!("You are {} years old", person.age);
}