use std::env; // crate that contains command line argument iterator
use std::process; // so we can manually exit
use io_project::Config;

fn main() {
    let args: Vec<String> =  env::args().collect(); // args() is an iterator. collect() groups them
    println!("{:?}", args);
    // run this with 'cargo run first_arg second_arg'
    // first thing printed is the name of binary created by this program
    // the following items in the vector are the command line args, if any

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // print to std error with eprintln!()
        process::exit(1);
    });

    println!("searching for: {}", config.query);
    println!("searching in: {}", config.filename);

    if let Err(e) = io_project::run(config) {
        println!("Problem calling run():\n{}", e);
        process::exit(1);
    }
}