use std::{fs};
use std::error::Error;

#[derive(Default)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let mut config: Config = Default::default();
        config.query = args[1].clone();
        config.filename = args[2].clone();
        config.set_flags(&args[3..]);
        Ok(config)
    }

    fn set_flags(&mut self, args: &[String]) {
        let flags = args.iter()
            .filter(|f| f.as_bytes()[0] == "-".as_bytes()[0]);
        println!("flags: {:?}", flags);
    }
}



pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        };
    };

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "DUCT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents))
    }

    #[test]
    fn no_results_with_mismatched_case() {
        let query = "DUCT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_ne!(vec!["safe, fast, productive."], search(query, contents))
    }
}