use std::{env, error::Error, fs, process};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_text = fs::read_to_string(&config.path)?;

    // ignore case or not check
    let res = if config.ignore_case {
        search_case_insensitive(&config.query, &file_text)
    } else {
        search(&config.query, &file_text)
    };

    res.iter()
        .for_each(|(i, line)| println!("{}:{line}", i + 1)); // print each line in res

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();

    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    // pub fn build(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("mg: minigrep requires @ least 1 pattern to exec a search");
    //     }
    //     let query = args[args.len() - 2].clone();
    //     let path = args[args.len() - 1].clone();
    //     let mut ignore_case = env::var("IGNORE_CASE").is_ok_and(|val| val == "1");
    //     // iterate through potential flags slice and match cases
    //     args[1..args.len() - 2]
    //         .iter()
    //         .for_each(|flag| match flag.as_str() {
    //             "-U" => ignore_case = true,
    //             flag => {
    //                 eprintln!("mg: unrecognized flag {flag}");
    //                 process::exit(1);
    //             }
    //         });
    //     Ok(Config {
    //         query,
    //         path,
    //         ignore_case,
    //     })
    // }
    pub fn build<T: Iterator<Item = String>>(mut args: T) -> Result<Config, &'static str> {
        // TODO implement error msg when no arguments given to minigrep. "mg: minigrep requires @ least 1 pattern to exec a search"
        // TODO implement a check for flags
        // TODO implement ignore_case search with -U flag
        // TODO if only one argument given, search every file in current directory
        args.next();
        let query = match args.next() {
            Some(q) => q,
            None => return Err("mg: missing query"),
        };
        let path = match args.next() {
            Some(p) => p,
            None => return Err("mg: missing text to query"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok_and(|val| val == "1");  // mutable for later
        Ok(Config {
            query,
            path,
            ignore_case,
        })
    }
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
            vec![(1, "safe, fast, productive.")],
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
            vec![(0, "Rust:"), (3, "Trust me.")],
            search_case_insensitive(query, contents)
        );
    }
}
