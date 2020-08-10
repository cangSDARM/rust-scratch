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

  //while it
  {
    //只要模式匹配就一直进行 while 循环
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
      println!("{}", top);
    }
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
    Err(_) => println!("err happened"),
    // _ 表示忽略该参数
    //参数也可以是一个具体的值
  };
  //注意的是，match可以返回值
  //match的默认值(default)可以随意起名字，但是该名字需要用到。不用的话使用 _ 代替

  //march的模式守卫(match guard)
  let (x, y) = (false, true);
  match Some(4){
    Some(n) if n < 5 => println!("less than 5"),
    Some(n) if x => println!("greater than 5, and y is true"),
    10 | 11 | 12 if y => { println!("equals 10 or 11 or 12, and y is true") },
    _ => (),
  }

  //@ 绑定
  //可以在一个模式中同时测试和保存变量值
  enum Massage { Hello { id: i32 }, }
  let msg = Massage::Hello {id: 5};
  match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
      //绑定到 id_variable 并且测试其是否在 [3, 7] 之间
      println!("Found an id in range: {}", id_variable)
    },
    _ => (),
  }

  //if let
  {
    let some = Some(1);
    let age: Result<u8, _> = "34".parse();
    let is_tuesday = false;

    //每一个 if let 都是match的语法糖, 但不同的是, if let不用穷尽所有可能值
    //if let 可以和其它if let混用
    //  也可以和普通if else混用
    if let Some(n) = some {
      print!("{}", n);
    } else if is_tuesday {
      ();
    } else if let Ok(age) = age {
      //这里括号内的age是Ok(age) shadow age后的值
      if age > 30 {
        println!("Using purple as the background color");
      }
    } else {
      ();
    }
  };
}
