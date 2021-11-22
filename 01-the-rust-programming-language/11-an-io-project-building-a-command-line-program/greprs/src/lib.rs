use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

//** Definition ****************************************************************
//== Terminal args =============================================================
pub struct Config<'init> {
    pub query: &'init str,
    pub filename: &'init str,
    pub case_sensitive: bool,
}

impl<'init> Config<'init> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // Index bound check
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Args
        let query = &args[1];
        let filename = &args[2];

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
    let mut f = File::open(config.filename)?;
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
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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
