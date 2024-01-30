use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("searching for {}", query);
    println!("in file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    println!("with text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = &args[1].clone();
        let file_path = &args[2].clone();

        Config {
            query, 
            file_path
        }
    }
    
}
