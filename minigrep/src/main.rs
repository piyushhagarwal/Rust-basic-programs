// to read the values of command line arguments we pass to it, we’ll need the std::env::args function
use std::env; 
use std::process;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect(); //the collect method on an iterator to turn it into a collection, such as a vector
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments {err}");
        process::exit(1);
    });
    println!("Searching for {}",config.query);
    println!("The file_path is {}",config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    
}

