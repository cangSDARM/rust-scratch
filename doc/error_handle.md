# 错误处理
Rust 将错误组合成两个主要类别：
1. 可恢复错误，使用`Result<T, E>`处理
2. 不可恢复错误，使用`panic!`处理
## 不可恢复
默认Rust会处理自己遗留的内容，可以选择直接终止把处理权上抛给系统
```yml
# Cargo.rs
[profile.release]
panic = 'abort'
```

```rs
panic!("Info");
```

## 可恢复
Result(enum)定义了两个成员
- Ok(T)
- Err(E)
```rs
use std::fs::File;
use std::io::ErrorKind;

let f:Result<std::fs::File, std::io::Error> = File::open("hello.txt");
let f = match f {
  Ok(file) => file,
  Err(e) => match e.kind() {
    ErrorKind::NotFound =>  match File::create("hello.txt") { 
      Ok(fc) => fc,
      Err(e) => panic!("{:?}", e),
    },
    other => panic!("{:?}", other)
  }
};

//闭包形式，效果同上
let f = File::open("hello.txt").map_err(|error| {
  if error.kind() == ErrorKind::NotFound {
    File::create("hello.txt").unwrap_or_else(|error| {
      panic!("{:?}", error);
    })
  } else {
    panic!("{:?}", error);
  }
});
```
> 任意类型的错误：`Box<dyn Error>`

**Result中的简便封装**
```rs
//Ok 返回值；Err painc
f = File::open("hello.txt").unwrap();

//更友好的信息
File::open("hello.txt").expect("Failed to open");
```

**错误也可以上抛**
```rs
use std::fs;
use std::fs::Read;
use std::io;

//调用时被上抛
fn read_username_from_file() -> Result<String, io::Error> {  
  let f = File::open("hello.txt");
  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };
  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}

//语法糖
fn read_username_from_file() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?; //Ok 继续；Err 返回
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}

//More... tricky
fn read_username_from_file() -> Result<String, io::Error> { 
  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}

//Right way. For this example
fn read_username_from_file() -> Result<String, io::Error> {   
  fs::read_to_string("hello.txt")
}
```
> 正确称呼应该是传播(propagating)<br>
> **`?`只能被用于返回值类型为 Result 的函数**
