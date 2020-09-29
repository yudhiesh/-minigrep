use std::error::Error;
use std::fs;

// Write a test that fails and run it to make sure it fails for the reason you except
// Write or modify enough code to make the new test pass
// Refactor the code and make sure it passes
// Repeat the process

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    // Error in the form of a static lifetime as it is a str not a String
    pub fn parse_config(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments passed in!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
// Returns an error trait called Box which is dynamic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents)? {
        println!("{:?}", line);
    }
    Ok(())
}

// Contents is the thing that we need in order to find the query that is why its lifetime is
// connected to the return
// Query exist as long as contents is there
//
pub fn search<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>, String> {
    let mut results = Vec::new();
    let query_lowercase = query.to_lowercase();

    for line in contents.lines() {
        if line.contains(&query_lowercase) {
            results.push(line);
        }
    }
    match results.len() {
        0 => Err(String::from("No Match Found!")),
        _ => Ok(results),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(Ok(vec!["safe, fast, productive."]), search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "dUcT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(Ok(vec!["safe, fast, productive."]), search(query, contents));
    }
}
