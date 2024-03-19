use std::{fs, env, error::Error};
use std::env::Args;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next()  {
            Some(arg) => arg,
            None => return Err("find empty query field")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("find empty file name filed")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // old code (without iterators)
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    // new code
    // iterator example
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "tootoo";
        let contents = "\
poopoopoo grrra.
gucci.
loktar-ogar! TooToo is,
tootoolitarism.";

        assert_eq!(vec!["tootoolitarism."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "LOKTAR";
        let contents = "\
poopoopoo grrra.
gucci.
loktar-ogar!
tootoolitarism.
Lock this tar for get loktarization output.";

        assert_eq!(vec!["loktar-ogar!", "Lock this tar for get loktarization output."], search_case_insensitive(query, contents));
    }
}
