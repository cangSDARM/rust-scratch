# Rust
## Quick Look
print!  是宏
print   是函数

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
4. `fu func() ->i32 { 5 }`是有效的声明，函数func返回
5. `let y = { lex x = 3; x+1 }`是有效的声明，y的值是x+1即4

## 结构
- doc<br>
| [Cargo](./doc/cargo.md)<br>
| [类型系统](./doc/type_system.md)<br>
| [所有权Ownership](./doc/ownership.md)
- src<br>
| [基础语法](./src/main.rs)<br>
| [流程控制](./src/process_control.rs)<br>