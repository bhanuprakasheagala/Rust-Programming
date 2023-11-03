fn main(){
    let celsius: f32;
    println!("Enter temperature in Fahrenhiet: ");
    let mut fahrenhiet = String::new();
    std::io::stdin().read_line(&mut fahrenhiet).expect("Failed to read line");
    let fahrenhiet: f32 = fahrenhiet.trim().parse().expect("Not a valid input");
    celsius = (fahrenhiet - 32.0) * 5.0/9.0;
    println!("Entered fahrenhiet number is: {}", fahrenhiet);
    println!("Celsius equalent to the entered Fahrenhiet: {}", celsius);
}