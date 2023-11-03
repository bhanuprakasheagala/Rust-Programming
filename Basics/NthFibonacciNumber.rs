fn main(){
    println!("Enter a number: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read the input");
    let input: i32 = input.trim().parse().expect("Not a positive number");
    let mut a = 0;
    let mut b = 1;
    if input == 0{
        println!("{}",a);
        std::process::exit(1);
    }
    let mut index = 2;
    while index <= input {
        let c = a + b;
        a = b;
        b = c;
        index += 1;
    }
    println!("Nth Fibonacci number: {}", b);
}