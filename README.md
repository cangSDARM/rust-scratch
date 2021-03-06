# Rust
## Quick Look
标志符以：<br>
1. 字母开头，后面跟字母、数字、_、None
2. _开头，但后面有字母、数字或\_
3. r#开头，后接rust关键字或字母，称为**row identifier**

### 语句和表达式
Rust中，语句没有返回值，而表达式有，这是他俩的重要区别<br>
且语句以分号结尾，而表达式无分号<br>
1. `let y = 5;`是一个语句
2. `fn func()`是一个语句
3. `{}`是一个表达式
4. `fu func() ->i32 { 10 }`是有效的声明，函数func返回10
5. `let y = { lex x = 3; x+1 }`是有效的声明，y的值是x+1即4

### Rust中的空值
Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。<br>
这个枚举是`Option<T>`<br>
Option有两个可能值，Some和None
```rust
let null: Option<i32> = None;
let some: Option<i32> = Some(12);
```
> 为了拥有一个可能为空的值，你必须要显式的将其放入对应类型的`Option<T>`中。接着，当使用这个值时，必须明确的处理值为空的情况。只要一个值不是`Option<T>`类型，你就可以安全的认定它的值不为空

**为了避免和None值计算或调用，Rust将抛出编译错误**
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;  //Error!
```
> 本质原因是：他俩类型不同

## 结构
- doc<br>
| [Cargo](./doc/cargo.md)<br>
| [模块化](./doc/modularize.md)<br>
| [基础数据类型](./doc/base_type.md)<br>
| [高级数据类型](./doc/advanced_type.md)<br>
| [所有权Ownership](./doc/ownership.md)<br>
| [结构体(Rust中的对象)](./doc/structure.md)<br>
| [枚举(Rust中对结构体的更高级封装?)](./doc/enums.md)<br>
| [错误处理](./doc/error_handle.md)<br>
| [自动化测试](./doc/auto_testing.md)<br>
| [模式匹配](./doc/patterns.md)<br>
| [unsafe及extern块](./doc/unsafe.md)<br>
| [宏。宏小册, 2017ver](https://www.bookstack.cn/read/DaseinPhaos-tlborm-chinese/README.md)<br>
- src<br>
| [基础语法](./src/main.rs)<br>
| [流程控制](./src/process_control.rs)<br>
| [泛型](./src/generics.rs)、[trait](./src/traits.rs)以及[生命周期](./src/lifecricle.rs)<br>
| [匿名函数及闭包、函数指针](./src/anonymous_function.rs)<br>
| [迭代器](./src/iterator.rs)<br>
| [智能指针](./src/smart_pointer.rs)<br>
| [并发](./src/concurrent.rs)<br>
| [在Rust中"面向对象"](./src/oo.rs)<br>

## 更多参考
| [The Rustonomicon](https://github.com/rust-lang/nomicon)<br>
| [async book](https://github.com/rust-lang/async-book)<br>
| [Rust Cookbook](https://github.com/rust-lang-nursery/rust-cookbook)<br>
| [The Rust and WebAssembly Book](https://github.com/rustwasm/book)<br>
| [Learn Rust by writing Entirely Too Many Linked Lists](https://github.com/rust-unofficial/too-many-lists)<br>
| [The Cargo Book](https://github.com/rust-lang/cargo/tree/master/src/doc/src)<br>
| [Rust Compiler Error Index](https://doc.rust-lang.org/error-index.html)<br>
