# 自动化测试
[返回](../README.md)

Rust 中的测试就是带有`test`**属性**注解的函数

属性(attribute)是关于Rust代码片段的元数据
```rust
// cargo new --lib 生成的测试模块样例
fn main() {}
#[cfg(test)]
mod tests {
  use super::*; //通常test模块会引用所有的模块项

  #[test]
  fn it_works() {
    assert!(true, "info");
    //eq和ne要求比较对象实现`PartialEq`和`Debug`两个trait
    assert_eq!(2 + 2, 4);
    assert_ne!(1, 0);
  }
}
```

**除了检查代码是否返回期望的值之外，检查代码是否正确处理错误也是很重要的**
```rust
#[test]
#[should_panic(expected = "Info")]
fn guess(){
  Guess::new(200);  //should painc in Guess::new
}
```
> 注意，`should_painc`中的expected必须是**发生panic的信息的子串**。这样做是为了保证发生的painc是预期的

**也可以使用Result枚举来控制测试是否正常**
```rust
#[test]
fn rusults() -> Result<(), String>{
  if 2+2 == 4{
    Ok(())
  }else{
    Err(Stirng::from("Error happed"))
  }
}
```
> 注意，`should_painc`和`Result`不能混用

## 测试控制流程及分类
使用`cargo test`运行测试(带test宏的)，其包括

1. 普通测试，由`running X test`显示
2. 被忽略的，由`X ignored`显示
3. 被过滤的，由`X filtered out`显示
4. 性能测试，由`X measured`显示
5. 文档测试，由`Doc-tests`显示

### 控制测试流程
`cargo test`在测试模式下亦会编译代码并运行生成的测试二进制文件

可以将一部分命令行参数传递给`cargo test`，而将另外一部分传递给生成的测试二进制文件。为了分隔这两种参数，需要先列出传递给`cargo test`的参数，接着是`分隔符 --`，再之后是传递给测试二进制文件的参数。运行`cargo test --help`会提示`cargo test`的有关参数，而运行`cargo test -- --help`可以提示在`分隔符 --`之后使用的有关参数

**当有多个多个测试时，Rust 默认使用线程来并行运行**

若存在并行依赖，可以使用`cargo test -- --test-threads=1`进行线性测试

**默认情况下，当测试通过时，Rust 只提供说明测试通过的提示行；失败时打印相关的标准输出和提示信息**

若希望看到通过测试时的输出，使用`cargo test -- --nocapture`。而测试的输出和测试结果的输出将会是相互交叉的，这是由于测试是*并行运行*的

**如果你负责特定位置的代码，你可能会希望只运行与这些代码相关的测试**

```rust
#![allow(unused_variables)]
fn main() {
  pub fn add_two(a: i32) -> i32 {
      a + 2
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn add_two_and_two() {
          assert_eq!(4, add_two(2));
      }

      #[test]
      fn add_three_and_two() {
          assert_eq!(5, add_two(3));
      }

      #[test]
      fn one_hundred() {
          assert_eq!(102, add_two(100));
      }
  }
}
```

运行单个：`cargo test on_hundred`；运行多个`cargo test add`（运行了所有名字中带有 add 的测试）；运行某个模块`cargo test tests`

**也可以简单点，忽略那些你不想测试的**

```rust
#![allow(unused_variables)]
fn main() {
  #[test]
  #[ignore]
  fn expensive_test() {
      // 需要运行一个小时的代码
  }
}
```

当然其实你也可以反过来，只运行被ignore的测试。可以使用`cargo test -- --ignored`

## 测试的分类
Rust 社区倾向于根据测试的两个主要分类来考虑问题：**单元测试（unit tests）**与**集成测试（integration tests）**。单元测试倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口。而集成测试对于你的库来说则完全是外部的。它们与其他外部代码一样，通过相同的方式使用你的代码，只测试公有接口而且每个测试都有可能会测试多个模块。

### 单元测试
单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中。规范是在每个文件中创建包含测试函数的`tests`模块，并使用`cfg(test)`标注模块

> 测试模块的`#[cfg(test)]`注解(configuration，它告诉 Rust 其之后的项只应该被包含进特定配置选项中)告诉 Rust 只在执行`cargo test`时才编译和运行测试代码，而在运行`cargo build`时不这么做

且注意，Rust 是可以运行私有函数或属性的。

### 集成测试
为了编写集成测试，需要在项目根目录创建一个 tests 目录，与 src 同级。Cargo 知道如何去寻找这个目录中的集成测试文件。接着可以随意在这个目录中创建任意多的测试文件，Cargo 会将每一个文件当作单独的 crate 来编译

> 与单元测试不同，我们需要在文件顶部添加`use XXX`。这是因为每一个 tests 目录中的**测试文件都是完全独立的 crate**，所以需要在每一个文件中导入库

并不需要将`tests/**.rs`中的任何代码标注为`#[cfg(test)]`。 tests 文件夹在 Cargo 中是一个特殊的文件夹， Cargo 只会在运行`cargo test`时编译这个目录中的文件

不同于单元测试，如果想要运行某个单独的测试，使用`cargo test`的`--test`*后*跟文件的名称来运行某个特定集成测试文件中的所有测试。如`cargo test --test integration_test`

**tests 目录中的文件不能像 src 中的文件那样共享相同的行为**

如，想共享一个 common.rs，为了不让 common 出现在测试输出中，需要创建 `tests/common/mod.rs`，而不是创建 tests/common.rs 。这是一种 Rust 的命名规范，这样命名告诉 Rust 不要将 common 看作一个集成测试文件，测试输出中将不会出现这一部分。tests 目录中的**子目录**不会被作为单独的 crate 编译或作为一个测试结果部分出现在测试输出中，然后便可以使用`mod common`来使用了

其次，如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，这样就不可能在 tests 目录创建集成测试并使用 extern crate 导入 src/main.rs 中定义的函数。原因在于二进制 crate 只意在单独运行
