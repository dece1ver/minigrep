use std::{error::Error, fs, env};

const COMMAND_CASE_INSENSITIVE: &str = "-i";

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        let commands = [COMMAND_CASE_INSENSITIVE];
        if args.len() < 3 {
            return Err(format!("Недостаточно агрументов - введено {}, необходимо минимум 2.", args.len() - 1));
        }
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        if args.len() >= 4 {
            for arg in args.iter().skip(3) {
                if commands.contains(&arg.as_str()){
                    if arg == COMMAND_CASE_INSENSITIVE {
                        case_sensitive = false;
                    }
                } else {
                    return Err(format!("Некорректный аргумент \"{}\"", arg));
                }
            }
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename, case_sensitive })
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
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
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
