# 数据类型
[返回](../README.md)

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

## 通用集合类型
这里指std中的类型。这些类型都是保存在堆上
### String
String和&str都是以`utf-8`格式所保存(因此涉及到多字节的问题)<br>
这种问题得自己处理
```rs
//字面量
let s = "hello";  //将硬编码到程序中
// s类型是&str(严格说是 &'static str )
// 因为其生命周期为整个程序期间，参看(../src/lifecricle.rs)
let st = s.to_string(); //字面量需要to_string后才是String

//String
let mut s = String::from("hello");  //存储在堆中
let mut s = String::with_capacity(10);  //申请容量为10字节的内存
```
> Rust中，String是对`Vec<u8>`对一个封装

**String的增删改查**
```rs
s.push('s');    //push(char)
s.push_str("ss");   //push(&str)

let s3 = s + &s2;   //注意 s 被移动了，不能再使用。其中 s2 (&String)被强转为&str

let s4 = format!("{}{}{}", s, s2, s3);  //这不会获取任何参数的所有权

// 1. Bad way
let s4 = &"你好"[0..2]   //双字节，s4将是 你
let s5 = &"你好"[0..1]  //Error! utf8问题
// 2. Good way
for i in &s.chars() {}  //基本上是对于双字节的两字节一个 i，以此类推
for i in &s.bytes() {}  //每字节一个 i，返回 u8 类型
```
> Rust不允许：`s[1]`，因为utf8和查找效率问题

### Vec\<T>
```rs
let v:Vec<i32> = Vec::new();

let mut v = vec![1, 2, 3];  //使用宏创建
```
> 若要保存不同类型的，请先定义包含所有需要保存类型的枚举<br>
> 如果无法枚举需要保存的类型，请使用triat

**Vec的增删改查**
```rs
//增
v.push(1);  //有mut的才行

//删
v.pop();    //弹出最后一个，同样需要mut

//改
v[1] = 2;   //有mut才行
for i in &mut v{
  *i += 12; // * 是`解引用`
}

//查
// 1. by索引
let i = &v[1];
// 2. by get
let i = match v.get(2) {
  Some(n) => n,
  None => {
    print!("none");
    -1
  }
};
```
> 注意，**有增(&, 非&mut)则不能有查，有查则不能有增**
### HashMap
```rs
use std::collections::HashMap;

let mut hmap = HashMap::new();

//从Vec的转换
let value = vec![1, 2];
let hmap: HashMap<_, _> = vec!["k1", "k2"].iter().zip(value).collect();
```
> HashMap和Vec一样，是同质的(即键的类型必须一致，值的类型也必须一致)
> _ 表示留给 Rust 自动推导

**HashMap的增删改查**
```rs
//覆盖
hmap.insert(String::from("AA"), 12);
//没有时更新
hmap.entry(String::from("AA")).or_insert(10);
//根据旧值修改
let count = hmap.entry("AA").or_insert(0);
*count += 1;    //有就对旧值+1

let s = String::from();
let i:Option<i32> = hmap.get(&s);

for (key, value) in &hmap {}
```

**一旦键值对被插入后就为 HashMap 所拥有**<br>
**如果将引用插入 HashMap，这些值本身将不会被移动进 HashMap。但是这些引用指向的值必须在 HashMap 有效时也是有效的**

## 特殊类型
### Newtype
[Newtype模式](../src/traits.rs)

### 类型别名
类型别名(type alias)意味着别名和真实类型是同义词(synonym), 这两个类型是完全可以替代使用的

类型别名的主要用途是减少重复
```rust
type Thunk = Box<dyn Fn() + Send + 'static>;
let f: Thunk = Box::new(|| println!("hi"));
fn toke() -> Thunk {}
fn gave(f: Thunk) {}
```

类型别名系统类似于Typescript中的, 也可以用泛型
```rust
type Result<T> = std::result::Result<T, std::io::Error>;
```

### Nevertype
rust中也存在一个 never 的类型(称为empty type), 即`!`

Nevertype 表示一个函数**从不返回值**(和[void](../src/main.rs)区别, void返回void, void确实是个类型), 这种从不返回值的函数叫做发散函数(diverging functions)
```rust
fn bar() -> ! {}
```

Nevertype 可以**强转为任意类型**
```rust
painc!("painc");    //painc 发生了强转
loop {
  print!("and ever ");    //print 发生了强转, 因而会产生死循环
}
```

Nevertype 常常用来绕过类型系统检查, 如`match`分支中
```rust
// match 分支要求返回类型相同. 这里`continue`返回了个Nevertype
let guess = match guess.trim().parse() {
  Ok(_) => 5,
  Err(_) => continue,
}
```

### 动态大小的类型
**Rust编译器需要知道每个函数的返回类型需要多少空间**, 这意味着所有函数都必须返回一个具体类型

但如同`str`, 直到运行时都不知道大小的类型, Rust 也有部分动态大小类型(dynamically sized types, DST)
```rust
let s: str = "str?";
```
> 但DST的变量**是无法直接声明的**, 上面的 s 的类型应该是`&str`. `&str`的长度是固定的(usize的两倍), 存储了字符串长度及头地址

所以, 动态大小类型的总是满足: 必须将动态大小类型的值置于某种指针之后. 而Rust的解决办法通常是用一些额外的元信息来储存动态信息的大小

#### dyn
`dyn`关键字用于附加一些元信息来扩展指针(如长度等), 来将动态大小约束为固定大小

通常有: `&dyn Trait`, `Box<dyn Trait>` 或 `Rc<dyn Trait>`:
```rs
// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}
```

不是所有的 trait 都能使用 dyn, 只有**对象安全**的才行:

- 返回值类型不为`Self`
- 方法没有任何泛型类型参数

#### Sized
对于自己声明的Trait, Rust使用`Sized` trait来限制Trait.

这个 trait 使得编译器在编译时就知道类型实现时的大小. 而且对于泛型来讲, 它是隐式实现的
```rust
fn generic<T>(t: T) { }
//In compiler time:
fn generic<T: Sized>(t: T) {}
```

这个限制可以使用`?Trait`来放宽
```rust
fn generic_dst<T: ?Sized>(t: &T) {}
```
> 放宽后的参数需要约束, 这里选择了T的引用

[更多细节 RFC255](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md)
