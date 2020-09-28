# 模式
模式是 Rust 中特殊的语法, 它用于匹配类型中的结构, 无论类型是简单还是复杂

### 模式可以存在的位置
1. match 分支
2. if let 条件表达式
3. while let 条件循环
4. for 循环
5. let 语句
6. 函数参数

> 1-4参看(../src/process_control.rs), 5-6参看(../src/main.rs)

### 两种形式
1. 不可反驳的(irrefutable): 能匹配任何传递的可能值的模式
2. 可反驳的(refutable): 对某些可能的值进行匹配会失败的模式

函数参数, let 语句和 for 循环只能接受不可反驳的模式. 因为若是可反驳的, 则程序的结果无法正确推断. <br>
if let 和 while let 表达式只能接受可反驳的模式. 因为根据定义他们需要处理可能的失败: 根据成功或失败执行不同的操作

### 语法
#### 匹配字面值
```rs
let x = 1;
match x {
  1 => println!("one"),
  _ => println!("other"),
}
```

#### 匹配命名变量
```rs
let mut s = String::new();
match io::stdin().read_line(&mut s) {
  Ok(n) => println!("read {} bytes", n),
  Err(_) => println!("err happend"),
};
```

#### 多个模式
```rs
let x = 1;
match x {
  1 | 2 => println!("one or two"),  //1 or 2
  3 => println!("three"),
  _ => println!("anything"),
}
```

#### 范围匹配
只能用于 数字/char 类型
```rs
let x = 5;
match x {
  1..=5 => println!("one through five"),    //[1, 5]
  'a'..'z' => println!("march char"),   //char类型
  _ => println!("something else"),
}
```

#### 解构
###### 结构体
```rs
struct Point {
  x: i32,
  y: i32,
}
let p = Point { x: 0, y: 7 };
let Point { x: a, y: b } = p;

match p {
   Point { x, y: 0 } => println!("On the x axis at {}", x),
   Point { x: 0, y } => println!("On the y axis at {}", y),
   Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}
```

###### 枚举
```rs
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

match msg {
  Message::Quit => {
    println!("The Quit variant has no data to destructure.")
  }
  Message::Move { x, y } => {
    println!("Move in the {} and {}", x, y);
  }
  Message::Write(text) => println!("Text message: {}", text),
  Message::ChangeColor(r, g, b) => {
    println!("Change the color to ({}, {}, {})", r, g, b)
  }
}
```

###### 嵌套的解构
```rs
// 嵌套的枚举和结构体
match msg {
  Message::ChangeColor(Color::Rgb(r, g, b)) => {
    println!("Change the color to ({}, {}, {})", r, g, b)
  }
  Message::ChangeColor(Color::Hsv(h, s, v)) => {
    println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
  }
  _ => ()
}

// 嵌套的结构体和元组
let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```

#### 忽略值
###### 忽略
```rs
let (x, _) = (1, 2);
let _x = 1;

//可以在函数参数中使用. 通常用于在实现 trait 时, 函数实现并不需要某个参数时
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
```

###### 省略
```rs
let (x, .., y) = (1, 2, 3, 4, 5, 6);
```
