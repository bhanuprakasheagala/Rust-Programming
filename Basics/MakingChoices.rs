fn main(){
    println!("What is your name?");
    let input = read_string();
    if input == ""{
        println!("You did not enter your name!!");
    }
    else if input == "Bhanuprakash" || input == "bhanuprakash" {
        println!("{} is a great name!", input);
    }
    else if input == "Marcel" || input == "marcel" {
        println!("I like the name {}", input);
    }
    else {
        println!("Your name is {}", input);
    }
}
fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read string");
    let cleaned_input = input.trim().to_string();

    cleaned_input
}