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

    // Add another association function which reads input from user and constructs a new `Person`
    fn new_from_input() -> Person {
        println!("First name: ");
        let first_name = read_string();
        println!("Last name: ");
        let last_name = read_string();
        println!("Age: ");
        let age = read_number();

        Person {
            first_name,
            last_name,
            age,
        }
    }
    /*
    We can add methods in the same impl block. The only difference between the associated function
    and a method is in the type signature. A method always takes &self as the first input parameter. 
    */
    fn print_person(&self) {
        println!("Hello {} {}", self.first_name, self.last_name);
        println!("You are {} years old", self.age);
    }
    /*
    Note the use of self.first_name, self.last_name, self.age in the print method to access the fields of the Person.
    */
}

fn main(){
    println!("What is your first name?");
    let first_name = read_string();
    println!("What is your last name?");
    let last_name = read_string();
    println!("What is your Age?");
    let age = read_number();

    // We've added a new() function that takes three input parameters: first_name, last_name, age and returns a new Person object.
    let person1 = Person::new(first_name, last_name, age); // User defined associated function
    //print_person(&person);
    person1.print_person();

    // User Defined associated function
    let person2 = Person::new_from_input();
    person2.print_person(); //Or print_person(&person);
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
/*
fn print_person(person: &Person){
    println!("Hello {} {}", person.first_name, person.last_name);
    println!("Your Age: {}", person.age);
}
*/