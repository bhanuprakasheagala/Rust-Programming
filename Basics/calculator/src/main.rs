use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = (args.nth(1).unwrap()).parse::<f32>().unwrap();
    // let operator = args.nth(0).unwrap().chars().next().unwrap(); OR
    let operator = (args.nth(0).unwrap()).parse::<char>().unwrap();
    let second = (args.nth(0).unwrap()).parse::<f32>().unwrap();
    let result = operate(operator, first, second);
    println!("{:?}", output(first, operator, second, result));
}

fn operate(operator: char, first: f32, second: f32) -> f32 {
   
    /*
    if operator == '+' {
        first + second
    }
    else if operator == '-' {
        first - second
    }
    else if operator == '*' {
        first * second
    }
    else{
        first / second
    }*/

    match operator {
        '+' => first + second,
        '-' => first - second,
        '*' | 'x' | 'X' => first * second,  //we can use 1 x 2 or 1 X 2 : extending
        '/' => first / second,
        _ => panic!("Invalid operator...") //default or base case : Must be there
    }

}
fn output(first: f32, operator: char, second: f32, result: f32) -> String {
    format!("{} {} {} = {}", first, operator, second, result)
}