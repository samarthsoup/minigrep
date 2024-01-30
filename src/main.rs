use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

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

fn parse_config(args: &[String]) -> Config {
    let query = &args[1].clone();
    let file_path = &args[2].clone();

    Config {
        query, 
        file_path
    }
}
