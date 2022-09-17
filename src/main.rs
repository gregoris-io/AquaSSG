use std::env;
use std::fs;



fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);


    //Saving argument values in variables
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    println!("Vector contains: {:?}",args);



    // Options
    let mut option = String::new();
    match option{
        "-v" | "--version"=>println!("version 0.0.1"),
        "-h" | "--version"=>println!("Welcome to the Static Site Generator", ""),
    }
    println!("The option selected: {}",option);
}
