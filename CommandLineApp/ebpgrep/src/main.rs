#![allow(unused)]

use clap::Parser;

#[derive(Parser)]
struct Cli{
    /// The pattern to look for
    pattern: String,
    /// The path to the file
    path: std::path::PathBuf,
}
fn main() {
    /*
    let _pattern = std::env::args().nth(1).expect("No pattern given!");
    println!("Entered pattern: {_pattern}");
    let _path = std::env::args().nth(2).expect("no path given");
    println!("Entered path: {_path}");
    */
    let args = Cli::parse();

    // Opening the file we got
    let content = std::fs::read_to_string(&args.path).expect("Could not read file");

    // Iterate over the lines and print each one that contains our pattern
    for line in content.lines() {
        if line.contains(&args.pattern){
            println!("{}", line);
        }
    }
}
