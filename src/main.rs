use std::env;
use std::fs;
use std::process;

// fn main() {
//     let args:Vec<String>=env::args().collect();
//     let query=&args[1];
//     let filename = &args[2];

//     println!("Searching for {query}");
//     println!("in file: {filename}");

//     let contents=fs::read_to_string(filename).expect("Unable to read a file");

//     println!("With text\n{contents}")
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("in file: {}", config.filename);

    run(config)
}

struct Config {
    query: String,
    filename: String,
}

/**
 * takes env arguments and parse them into correct variables
 *
 * then returns them
 */
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn run(config:Config){
    
let contents = fs::read_to_string(config.filename).expect("Unable to read a file");

println!("With text\n{contents}")
}