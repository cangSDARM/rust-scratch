### How to restrcuture your crate
```rs
//Original
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
    let contents = fs::read_to_string(config.file_name).expect("Something wen wrong reading the file");
    println!("File Content: {}", contents);
}

// Restrcuture: 1, seprate the steps
fn parse(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_name = &args[2];
    (query, file_name)
}
let (query, file_name) = parse(&args);

// Restrcuture: 2, Combination type
struct Config {
    query: String,
    file_name: String,
}
fn parse(args: &[String]) -> Config {
    Config {
        query: args[1].clone(),
        file_name: args[2].clone(),
    }
}
let config = parse(&args);

// Restrcuture: 3, Save the time
struct Config<'a> {
    query: &'a str,
    file_name: &'a str,
}
fn parse<'a>(args: &'a [String]) -> Config<'a> {
    //适用于 生命周期注解省略规则2, 'a可以忽略
    Config {
        query: &args[1],
        file_name: &args[2],
    }
}

// Restrcuture: 4, Scope the type of behavior
struct Config<'a> {
    query: &'a str,
    file_name: &'a str,
}
impl<'a> Config<'a> {
    fn new(args: &[String]) -> Config {
        Config { query: &args[1], file_name: &args[2] }
    }
}

// Restrcuture: 5, Handle Error and panic it
fn new(args: &[String]) -> Config {
    if args.len() != 3 {
        panic!(
            "not enough arguments. expect recieve 2, but got {}",
            args.len()
        );
    }
    Config { query: &args[1], file_name: &args[2] }
}

// Restrcuture: 6, Return Result rather than exit the process by itself
fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() != 3 {
        return Err("not enough arguments");
    }
    Config { query: &args[1], file_name: &args[2] }
}
let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
});

// Restrcuture: 7, Use mod

// Restrcuture: 8, Remove println!, and write tests

// Restrcuture: 9, Use Env rather a row string

```
