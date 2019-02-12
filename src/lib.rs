use std::{fs, env, error::Error};

pub struct Config {
    pub pattern: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str> 
        where T: Iterator<Item = String>
    {
        //First arg is program name
        args.next();

        //Unpack args
        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("not enough arguments specified")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("not enough arguments specified")
        };

        //Search is case-insensitive iff CASE_INSENSITIVE=1
        let case_sensitive = match env::var("CASE_INSENSITIVE") {
            Ok(val) => if val.parse().unwrap_or(0) == 1 {
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
    contents.lines()
        .filter(|line| {line.contains(&pattern)})
        .collect()
}

pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();

    contents.lines()
        .filter(|line| {line.to_lowercase().contains(&pattern)})
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    //Struct for simulating command line arguments to Config
    struct TestArgs {
        data: Vec<String>,
        index: usize
    }

    impl TestArgs {
        pub fn new(pattern: &str, filename: &str) -> TestArgs {
            let mut data = vec![String::new()];
            data.push(String::from(pattern));
            data.push(String::from(filename));

            TestArgs{data, index: 0}
        }
    }

    impl Iterator for TestArgs {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                self.index += 1;
                Some(self.data[self.index - 1].to_string())
            } else {
                None
            }
        }
    } 

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
        //Remove the first argument
        let args = TestArgs::new("pattern", "filename").filter(|s| *s != String::new());

        let config = Config::new(args).unwrap_or_else(|err| {
            panic!("could not construct Config: {}", err);
        });
    }

    #[test]
    fn run_success() {
        let config = prepare_config(" ", "test.txt");

        if let Err(e) = run(config) {
            panic!("run failed: {}", e);
        }
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

    fn prepare_config(pattern: &str, filename: &str) -> Config {
        let args = TestArgs::new(pattern, filename);
        let config = Config::new(args).unwrap_or_else(|err| {
            panic!("could not construct Config: {}", err);
        });

        config
    }
}