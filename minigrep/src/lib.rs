use std::{env, error::Error, fs};

pub struct Config {
  pub query: String,
  pub file_name: String,
  pub sensitive: bool,
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    Ok(Config {
      query: match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a query string"),
      },
      file_name: match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file name"),
      },
      sensitive: !env::var("CASE_SENSITIVE").is_err(),
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_name)?;
  let results = if config.sensitive {
    search(config.query, &contents)
  } else {
    search_case_insensitive(config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

fn search_case_insensitive<'a>(query: String, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
    .collect()
}

fn search<'a>(query: String, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(&query))
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn case_sensitive() {
    let query = String::from("duct");
    let contents = "\
  Rust:
  safe, fast, productive.
  Pick three.
  Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents))
  }

  #[test]
  fn case_insensitive() {
    let query = String::from("rUsT");
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
