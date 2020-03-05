# Rust

## Cargo
cargo是rust的包(crates)管理工具

使用`cargo init --bin 项目名`或者`cargo new 项目名`可以创建一个默认的项目<br>
  *官方解释是，init从已有文件开始，new可以创建文件，实际上没看出区别*

1. `Cargo.toml`和`package.json`效果相同
2. `cargo run`和`npm run`效果相同
3. `cargo build`和`npm build`相同
4. `cargo build --release`会构建release包

### 安装依赖
1. 打开[crates.io](https://crates.io)
2. 搜索想要的依赖
3. 在`Cargo.toml`中的dependences下添加`依赖="依赖版本号"`，等同于在`package.json`中添加`依赖:"^依赖版本号"`
4. 手动或自动地运行`cargo build`

print!  是宏
print   是函数

标志符以：<br>
1. 字母开头，后面跟字母、数字、_、None
2. _开头，但后面有字母、数字或\_
3. r#开头，后接rust关键字或字母，称为**row identifier**

## 类型系统
### 标量(Scalar)类型
代表一个单独的值，Rust有四种基本标量类型<br>
1. **整型**
    |长度|有符号|无符号|
    |:-:|:-:|:-:|
    |8-bit|i8|u8|
    |16-bit|i16|u16|
    |32-bit|i32|u32|
    |64-bit|i64|u64|
    |arch|isize|usize|
    - 默认为i32
    - 其中，arch代表机器字节数。在64位机上代表64，32位上代表32
    - 在release中，Rust的溢出将循环。如u8类型导致256变成0，257变成1

    |字面值|例子|
    |:-:|:-:|
    |Decimal|98_000|
    |Hex|0xff|
    |Octal|0o77|
    |Binary|0b1111_0000|
    |Byte(u8 only)|b'A'|
    - 默认是十进制数(Decimal)
    - _代表可读性。没有其他含义
2. **浮点**
    - 有两类，`f32`和`f64`
    - 默认为f64
3. **布尔bool**
    - `true` / `false`
4. **字符char**
    - 默认为unicode标量类型。可以赋值为任意unicode字符
    - 使用`'`(单引号)包裹
### 复合(Compound)类型
用于将多个值组合为一个类型，Rust有两种基本复合类型<br>
1. **元祖tuple**
    - 例子：`let tup: (i32, f64, u8) = (40, 8.3, 1);`
    - 模式匹配来结构：`let (x, y, z) = tup;`
    - 索引：`let x = tup.0; let y = tup.1; let z = tup.3`
2. **数组array**
    - 例子：`let arr :[i32; 3] = [1, 2, 3];`
    - 数组长度是固定的(可变的是vector)
    - 所有内容的类型都是一致的
    - 数组Rust存在栈(stack)上
### 语句和表达式
Rust中，语句没有返回值，而表达式有，这是他俩的重要区别<br>
且语句以分号结尾，而表达式无分号<br>
1. `let y = 5;`是一个语句
2. `fn func()`是一个语句
3. `{}`是一个表达式
4. `fu func() ->i32 { 5 }`是有效的声明，函数func返回5
5. `let y = { lex x = 3; x+1 }`是有效的声明，y的值是x+1即4

## [所有权Ownership](./doc/ownership.md)