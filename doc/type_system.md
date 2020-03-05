# 类型系统
[return](../README.md)

## 栈上的数据
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

## 堆/硬编码的数据
### String
```rs
//字面量
let s = "hello";  //将硬编码到程序中，s类型是&str

//String
let mut s = String::from("hello");  //存储在堆中
s.push_str(", world")  //在后面追加值
```