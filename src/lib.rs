use std::{fs, env, error::Error};

pub struct Config {
    pub pattern: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let pattern = args[1].clone();
        let filename = args[2].clone();

        //Search is case-insensitive iff CASE_INSENSITIVE=1
        let case_sensitive = match env::var("CASE_INSENSITIVE") {
            Ok(val) => if val.parse().unwrap_or_else(|err| {0}) == 1 {
                false
            } else {
                true
            },
            Err(_) => true
        };

        Ok(Config {pattern, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.pattern, &contents)
    } else {
        search_case_insensitive(&config.pattern, &contents)
    };

    for line in results {
        println!("{}", line);
    } 

    Ok(())
}

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(pattern) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let pattern = pattern.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&pattern) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config() {
        let config = prepare_config("pattern", "filename");

        assert_eq!(config.pattern, "pattern");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    #[allow(unused_variables)]
    fn new_config_failure() {
        let mut args = prepare_args("pattern", "filename");
        //Remove an argument so Config constructor fails
        args.pop();

        let config = Config::new(&args).unwrap_or_else(|err| {
            panic!("could not construct Config: {}", err);
        });
    }

    #[test]
    #[should_panic(expected = "run failed")]
    fn run_failure() {
        let config = prepare_config("pattern", "n0t @ f!L3");

        if let Err(e) = run(config) {
            panic!("run failed: {}", e);
        }
    }

    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(pattern, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let pattern = "RuSt";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(pattern, contents)
        );
    }

    fn prepare_args(pattern: &str, filename: &str) -> Vec<String> {
        vec![String::new(), String::from(pattern), String::from(filename)]
    }

    fn prepare_config(pattern: &str, filename: &str) -> Config {
        let args = prepare_args(pattern, filename);
        let config = Config::new(&args).unwrap_or_else(|err| {
            panic!("could not construct Config: {}", err);
        });

        config
    }
}