use std::{env, process};

mod lib;
use lib::{run, Config};

fn main() {
    //env::args 返回环境变量的迭代器，collect转为Vec对象
    //在任意参数包含无效的Unicode字符(如 , - / 开头)时 panic，需要其他的使用args_os代替(但会根据平台变化)

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        //return Ok's value Or return a closure to handle error or something else
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
