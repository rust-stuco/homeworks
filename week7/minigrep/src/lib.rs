use std::error::Error;
use std::fs;

#[derive(Debug)]
#[non_exhaustive]
enum Output {
    Lines,
    CountOccurrences,
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    output_type: Output,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case =
            args.contains(&"-i".to_string()) || args.contains(&"--ignore".to_string());
        let count_occurrences =
            args.contains(&"-c".to_string()) || args.contains(&"--count".to_string());

        if count_occurrences {
            Ok(Config {
                query,
                file_path,
                ignore_case,
                output_type: Output::CountOccurrences,
            })
        } else {
            Ok(Config {
                query,
                file_path,
                ignore_case,
                output_type: Output::Lines,
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search::<false>(&config.query, &contents)
    } else {
        search::<true>(&config.query, &contents)
    };

    match config.output_type {
        Output::CountOccurrences => println!("{}", results.len()),
        Output::Lines => {
            for (line_num, line_str) in results {
                println!("{}: {}", line_num, line_str);
            }
        }
    }

    Ok(())
}

pub fn search<'a, const CASE_SENSITIVE: bool>(
    query: &str,
    contents: &'a str,
) -> Vec<(usize, &'a str)> {
    let query = if CASE_SENSITIVE {
        query.to_string()
    } else {
        query.to_lowercase()
    };

    let mut results = Vec::new();

    for (line_num, line_str) in contents.lines().enumerate() {
        if (CASE_SENSITIVE && line_str.contains(&query))
            || (!CASE_SENSITIVE && line_str.to_lowercase().contains(&query))
        {
            results.push((line_num, line_str))
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
Pick three.";

        assert_eq!(
            vec![(1, "safe, fast, productive.")],
            search::<true>(query, contents)
        );
    }
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
            search::<true>(query, contents)
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
            search::<false>(query, contents)
        );
    }
}
