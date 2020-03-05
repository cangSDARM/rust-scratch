use std::cmp::Ordering;
// Ordering 也是枚举类型，包含 Less、Equal、Greater
use std::io;

fn ifing(num: u8) {
  let number = if num < 10 {
    //表达式Rust不会自动转换为bool，需要自己转换
    5
  } else if num < 20 {
    12
  } else {
    6
  };
  // if是一个表达式，因此这个是允许的
  // 但如果这么做，那么每个分支必须是相同类型的
  println!("{}", number);
}

fn looping() {
  let i = loop {
    println!("looping");

    break 1; //退出最近的loop，并赋值给变量(如果有的话)
  };
}

fn whileing() {
  while 1 < 2 {
    break;
  }
}

fn foring() {
  for i in [1, 2, 3].iter() {
    //遍历数组
    println!("{}", i);
  }

  for i in (1..4).rev() {
    //遍历range
    println!("{}", i);
  }
}

fn matching(num: i32) {
  match 1.cmp(&num) {
    //cmp比较对象
    Ordering::Less => println!("less"),
    Ordering::Equal => print!("equal"),
    Ordering::Greater => {
      print!("greater");
      print!("OK");
    }
  }
  // match 表示模式匹配，和switch相似。
  // 当 match 匹配时将自动break
  //FIXME: if break, how to continue? else, how break
  // 且 match 可以传递参数，如：
  let mut s = String::new();
  match io::stdin().read_line(&mut s) {
    Ok(n) => println!("read {} bytes", n),
    Err(_) => println!("err happend"),
    // _ 表示忽略该参数，若在 match 中使用，表示default情况
  }
}
