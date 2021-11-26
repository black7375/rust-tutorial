use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

//** Definition ****************************************************************
//== Terminal args =============================================================
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

//== File ======================================================================
// Never
// or use https://doc.rust-lang.org/std/primitive.never.html
// https://docs.rs/never/0.1.0/never/enum.Never.html
// enum Never { }

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(&config.filename)?;
        //.expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
        //.expect("Something went wrong reading the file");

    #[cfg(debug_assertions)]
    println!("With text:\n{}", contents);

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }
    else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

//== Search ====================================================================
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

//** Test **********************************************************************
//== Terminal args =============================================================
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
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
