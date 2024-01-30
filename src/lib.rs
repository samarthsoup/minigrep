use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        //args in the parameters of this fn is an iterator that iterates over a collection of Strings
        args.next(); //ignore the first element as its the executable binary
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a filepath"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok(); /*IGNORE_CASE=1 cargo run -- query filepath
        //run that command to set the env variable IGNORE_CASE to 1, which will mean that is_ok() returns true
        */

        Ok(Config {
            query, 
            file_path, 
            ignore_case
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
        
    let results = if config.ignore_case { //if ignore_case is true, run insensitive search
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };
    
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    contents
        .lines() //turns a string into an iterable with items of string separated by newline
        .filter(|line| line.contains(&query)) //keep only those elements that contain the query
        .collect() //consume the iterator and collect it into a Vec
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents    
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
