use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("in file {}", config.file_path);

    run(config);

}

run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("should have been able to read the file");
        
    println!("with text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = &args[1].clone();
        let file_path = &args[2].clone();

        Config {
            query, 
            file_path
        }
    }
    
}
