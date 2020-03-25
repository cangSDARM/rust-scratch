use std::cmp::Ordering;
// Ordering 也是枚举类型，包含 Less、Equal、Greater
use std::io;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn looping() {
  let _i = loop {
    println!("looping");

    break 1; //退出最近的loop，并赋值给变量(如果有的话)
  };
}

#[allow(dead_code)]
fn whileing() {
  while 1 < 2 {
    break;
  }
}

#[allow(dead_code)]
fn foring() {
  //遍历数组
  for i in [1, 2, 3].iter() {
    //.iter()返回每个元素
    println!("{}", i);
  }
  //遍历range
  for i in (1..4).rev() {
    println!("{}", i);
  }
  //遍历字符串
  let bytes = "sss".as_bytes(); //将字符串转为字节数组
  for (i, &item) in bytes.iter().enumerate() {
    //enumerate返回包装好的元素对象，更方便
    //&提取元素引用，而不转移所有权
    if item == b' ' {
      print!("{}", i);
    }
  }
}

#[allow(dead_code)]
fn matching(num: i32) {
  match 1.cmp(&num) {
    //cmp比较对象
    Ordering::Less => println!("less"),
    Ordering::Equal => print!("equal"),
    Ordering::Greater => {
      print!("greater");
      print!("OK");
    }
  };
  // match 表示模式匹配，和switch相似。
  // 当 match 匹配时将自动break
  // 且 match 可以传递参数，如：
  // 注意 match块 后的逗号。这会将这个match变成一个语句，而不是表达式
  let mut s = String::new();
  match io::stdin().read_line(&mut s) {
    Ok(n) => println!("read {} bytes", n),
    Err(_) => println!("err happend"),
    // _ 表示忽略该参数
    //参数也可以是一个具体的值
  };
  //注意的是，match可以返回值
  //match的默认值(default)可以随意起名字，但是该名字需要用到。不用的话使用 _ 代替

  //if let
  {
    //对于只有一个分支的match的语法糖
    let some = Some(1);
    match some {
      Some(n) => print!("{}", n),
      _ => (),
    }

    //等价于
    if let Some(n) = some {
      print!("{}", n);
    } else {
      ()
    }
  };
}
