use std::{env, error::Error, fs};

pub struct Config<'a> {
  pub query: &'a str,
  pub file_name: &'a str,
  pub sensitive: bool,
}

impl<'a> Config<'a> {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() != 3 {
      return Err("not enough arguments");
    }

    Ok(Config {
      query: &args[1],
      file_name: &args[2],
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line.trim());
    }
  }

  results
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line.trim());
    }
  }
  results
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
  Rust:
  safe, fast, productive.
  Pick three.
  Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents))
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
