use std::error::Error;
use std::fs;

// declare nearly everything as public so it can be seen and used in main.rs
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments given.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        return Ok(Config {query, filename});
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // open the file poem.txt, which we should have passed as second_arg
    //let contents = fs::read_to_string(config.filename).expect("Something went wrong reading file");
    let contents = fs::read_to_string(config.filename)?; // instead of panic in expect, return error 

    let results = search(&config.query, &contents); 
    //println!("{:?}", results);

    // or print one line at a time
    for line in results {
        println!("{}", line);
    }

    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
 // manually annotate lifetime of returned reference to match 'contents'

 let mut results = Vec::new();
 for line in contents.lines() {
     if line.contains(query) {
         results.push(line);
     }
 }
 return results;
}