# Rust

## Quick Look

标志符以：<br>

1. 字母开头，后面跟字母、数字、\_、None
2. \_开头，但后面有字母、数字或\_
3. r#开头，后接 rust 关键字或字母，称为**row identifier**

### 语句和表达式

Rust 中，语句没有返回值，而表达式有，这是他俩的重要区别<br>
且语句以分号结尾，而表达式无分号<br>

1. `let y = 5;`是一个语句
2. `fn func()`是一个语句
3. `{}`是一个表达式
4. `fu func() ->i32 { 10 }`是有效的声明，函数 func 返回 10
5. `let y = { lex x = 3; x+1 }`是有效的声明，y 的值是 x+1 即 4

### Rust 中的空值

Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。<br>
这个枚举是`Option<T>`<br>
Option 有两个可能值，Some 和 None

```rust
let null: Option<i32> = None;
let some: Option<i32> = Some(12);
```

> 为了拥有一个可能为空的值，你必须要显式的将其放入对应类型的`Option<T>`中。接着，当使用这个值时，必须明确的处理值为空的情况。只要一个值不是`Option<T>`类型，你就可以安全的认定它的值不为空

**为了避免和 None 值计算或调用，Rust 将抛出编译错误**

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;  //Error!
```

> 本质原因是：他俩类型不同

## 结构

- doc
  - [Cargo](./doc/cargo.md)
  - [模块化](./doc/modularize.md)
  - [基础数据类型](./doc/base_type.md)
  - [高级数据类型](./doc/advanced_type.md)
  - [所有权 Ownership](./doc/ownership.md)
  - [结构体(Rust 中的对象)](./doc/structure.md)
  - [枚举(Rust 中对结构体的更高级封装?)](./doc/enums.md)
  - [错误处理](./doc/error_handle.md)
  - [自动化测试](./doc/auto_testing.md)
  - [模式匹配](./doc/patterns.md)
  - [unsafe 及 extern 块](./doc/unsafe.md)
  - [宏。宏小册, 2017ver](https://www.bookstack.cn/read/DaseinPhaos-tlborm-chinese/README.md)
- src
  - [基础语法](./src/main.rs)
  - [流程控制](./src/process_control.rs)
  - [trait](./src/traits.rs)、[生命周期](./src/lifecricle.rs)以及[泛型](./src/generics.rs)
  - [匿名函数及闭包、函数指针](./src/anonymous_function.rs)
  - [迭代器](./src/iterator.rs)
  - [智能指针](./src/smart_pointer.rs)
  - 并发和并行
    - [线程，并行](./src/threads.rs)
    - [异步，并发](./src/async.rs)
  - [在 Rust 中"面向对象"](./src/oo.rs)

## 更多参考

- [The Rustonomicon](https://github.com/rust-lang/nomicon)
- [async book](https://github.com/rust-lang/async-book)
- [Rust Cookbook](https://github.com/rust-lang-nursery/rust-cookbook)
- [The Rust and WebAssembly Book](https://github.com/rustwasm/book)
- [Learn Rust by writing Entirely Too Many Linked Lists](https://github.com/rust-unofficial/too-many-lists)
- [The Cargo Book](https://github.com/rust-lang/cargo/tree/master/src/doc/src)
- [Rust Compiler Error Index](https://doc.rust-lang.org/error-index.html)
- [Some helpful hints](https://chercher.tech/rust/errors-in-rust)
- [Gof Design Patterns](https://github.com/lpxxn/rust-design-pattern)
- [Godot Rust Example](https://github.com/fringelin/godot_rust_example)
- MS Rust<br>
  | [Intro](https://docs.microsoft.com/en-us/windows/dev-environment/rust/rust-for-windows)<br>
  | [Doc](https://microsoft.github.io/windows-docs-rs/doc)<br>
  | [Example](https://github.com/kekeimiku/msrs)<br>
- [Rust Magazine](https://rustmagazine.github.io/rust_magazine_2021/index.html)
- [Rust in embedded](https://docs.rust-embedded.org/)

## 例子

- [ip scanner(threads and mpsc)](https://github.com/tensor-programming/Rust_Port_Sniffer)
- [chat(server/client with raw TcpConnection)](https://github.com/tensor-programming/Rust_client-server_chat)
- [webpage crawler(async and threads)](https://github.com/tensor-programming/crawler_example)
