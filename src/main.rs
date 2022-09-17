use std::env;
use std::fs;
use clap::Parser;

mod cli;

use cli::AquaSST;

fn main() {

    let args = AquaSST::parse();

    //let args: Vec<String> = env::args().collect();
    
    // #[derive(Parser)]
    // struct Cli {
    //     //Pattern to look for
    //     pattern: String,
    //     // Path of file to read
    //     #[clap(parse(from_os_str))]
    //     path: std::path::PathBuf,
    // }

    //Saving argument values in variables
    // let mut action_option = &args[1];
    // let file_name = &args[2];
    // let file_path = &args[3];

    // println!("Processing {}", file_name);
    // println!("Processing {}", file_path);
    // //test contents
    // println!("Vector contains: {:?}",args);

    //Write



}
