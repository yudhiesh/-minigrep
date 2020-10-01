use std::env;
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
    pub fn parse_config(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query!"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a filename!"),
        };
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
    // Using an iterator you do not need to specify a vec and push the contents of it into
    // the vec again
    // Instead it can all be collected and put into a variable
    // Need to specify the type of results
    // Vec<_> is a generic type where the compiler will infer the type for you
    let results: Vec<_> = contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
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
